// Aster Ghidra export script. Build against the installed Ghidra API; run from GUI or analyzeHeadless.
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonArray;
import com.google.gson.JsonNull;
import com.google.gson.JsonObject;
import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.script.GhidraScript;
import ghidra.framework.Application;
import ghidra.program.model.data.Array;
import ghidra.program.model.data.DataType;
import ghidra.program.model.data.Structure;
import ghidra.program.model.data.Union;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionIterator;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.Parameter;
import ghidra.program.model.pcode.HighFunction;
import ghidra.program.model.pcode.PcodeBlockBasic;
import ghidra.program.model.pcode.PcodeOpAST;
import ghidra.program.model.pcode.Varnode;
import ghidra.program.model.symbol.Reference;
import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Iterator;
import javax.swing.JOptionPane;
import javax.swing.JScrollPane;
import javax.swing.JTextArea;

public class AsterExport extends GhidraScript {
    @Override
    protected void run() throws Exception {
        String[] args = getScriptArgs();
        File output;
        String selectedFunction = null;
        if (args.length > 0) {
            output = new File(args[0]);
            if (args.length > 1) selectedFunction = args[1];
        } else if (!isRunningHeadless()) {
            output = askFile("Export Ghidra analysis for Aster", "Export");
        } else {
            throw new IllegalArgumentException("headless usage: AsterExport.java <output.json> [function-name]");
        }
        if (output == null) throw new IllegalArgumentException("no output file selected");

        DecompInterface decompiler = new DecompInterface();
        decompiler.toggleSyntaxTree(true);
        decompiler.toggleCCode(true);
        if (!decompiler.openProgram(currentProgram)) {
            throw new IllegalStateException("could not open Ghidra program in decompiler: " + decompiler.getLastMessage());
        }

        JsonObject root = new JsonObject();
        root.addProperty("schemaVersion", 1);
        root.addProperty("producer", "Aster Ghidra exporter");
        root.addProperty("ghidraVersion", Application.getApplicationVersion());
        root.add("program", exportProgram());
        boolean selectionActive = currentSelection != null && !currentSelection.isEmpty();
        boolean scopeComplete = selectedFunction == null && !selectionActive;
        root.addProperty("scope", scopeComplete ? "program" : selectionActive ? "selection" : "function-filter");
        root.addProperty("scopeComplete", scopeComplete);
        root.addProperty("programFunctionCount", currentProgram.getFunctionManager().getFunctionCount());
        JsonArray functions = new JsonArray();
        FunctionIterator iterator = currentProgram.getFunctionManager().getFunctions(true);
        int visited = 0;
        try {
            while (iterator.hasNext()) {
                monitor.checkCancelled();
                Function function = iterator.next();
                if (selectedFunction != null && !selectedFunction.equals(function.getName())) continue;
                if (currentSelection != null && !currentSelection.isEmpty() && !currentSelection.intersects(function.getBody())) continue;
                functions.add(exportFunction(function, decompiler));
                visited++;
                monitor.setMessage("Exporting Ghidra function " + function.getName());
            }
        } finally {
            decompiler.dispose();
        }
        root.add("functions", functions);
        root.add("dataTypes", exportDataTypes());
        root.addProperty("functionCount", visited);
        root.addProperty("note", "P-code and recovered metadata are evidence, not proof of source semantics or memory safety.");

        Gson gson = new GsonBuilder().setPrettyPrinting().disableHtmlEscaping().create();
        Path target = output.toPath().toAbsolutePath();
        if (target.getParent() != null) Files.createDirectories(target.getParent());
        Files.writeString(target, gson.toJson(root), StandardCharsets.UTF_8);
        println("Aster Ghidra evidence exported: " + target + " (" + visited + " functions)");
        if (!isRunningHeadless()) showAsterPreview(target);
    }

    private void showAsterPreview(Path exportPath) throws Exception {
        String cli = System.getenv("ASTER_CLI");
        if (cli == null || cli.isBlank()) {
            println("Set ASTER_CLI to the built Aster executable to preview the supported Aster subset.");
            return;
        }
        String exportName = exportPath.getFileName().toString();
        int dot = exportName.lastIndexOf('.');
        String base = dot > 0 ? exportName.substring(0, dot) : exportName;
        Path asterPath = exportPath.resolveSibling(base + ".as");
        Path reportPath = exportPath.resolveSibling(base + ".ghidra-report.json");
        Process process = new ProcessBuilder(cli, "ghidra", "lower", exportPath.toString(), "-o", asterPath.toString())
            .redirectErrorStream(true).start();
        String output = new String(process.getInputStream().readAllBytes(), StandardCharsets.UTF_8);
        int status = process.waitFor();
        String source = Files.exists(asterPath) ? Files.readString(asterPath, StandardCharsets.UTF_8) : "No supported Aster source was generated.";
        JTextArea preview = new JTextArea("Lowering status: " + (status == 0 ? "complete for the supported subset" : "partial/rejected (exit " + status + ")")
            + "\nReport: " + reportPath + "\n" + output + "\n--- Aster preview ---\n" + source);
        preview.setEditable(false);
        preview.setCaretPosition(0);
        JOptionPane.showMessageDialog(null, new JScrollPane(preview), "Aster P-code preview", JOptionPane.INFORMATION_MESSAGE);
    }

    private JsonObject exportProgram() {
        JsonObject out = new JsonObject();
        out.addProperty("name", currentProgram.getName());
        out.addProperty("executablePath", currentProgram.getExecutablePath());
        out.addProperty("languageId", currentProgram.getLanguageID().getIdAsString());
        out.addProperty("compilerSpecId", currentProgram.getCompilerSpec().getCompilerSpecID().getIdAsString());
        out.addProperty("bigEndian", currentProgram.getLanguage().isBigEndian());
        out.addProperty("pointerSize", currentProgram.getDefaultPointerSize());
        return out;
    }

    private JsonObject exportFunction(Function function, DecompInterface decompiler) {
        JsonObject out = new JsonObject();
        out.addProperty("name", function.getName());
        out.addProperty("entry", function.getEntryPoint().toString());
        out.addProperty("bodyBytes", function.getBody().getNumAddresses());
        out.addProperty("callingConvention", function.getCallingConventionName());
        out.addProperty("isExternal", function.isExternal());
        out.addProperty("isThunk", function.isThunk());
        out.addProperty("noReturn", function.hasNoReturn());
        out.add("returnType", dataType(function.getReturnType()));
        JsonArray parameters = new JsonArray();
        for (Parameter parameter : function.getParameters()) {
            JsonObject p = new JsonObject();
            p.addProperty("name", parameter.getName());
            p.add("type", dataType(parameter.getDataType()));
            p.addProperty("sizeBytes", parameter.getLength());
            p.addProperty("storage", parameter.getVariableStorage().toString());
            parameters.add(p);
        }
        out.add("parameters", parameters);
        out.add("locals", exportLocals(function));

        DecompileResults result = decompiler.decompileFunction(function, 60, monitor);
        out.addProperty("decompileCompleted", result != null && result.decompileCompleted());
        if (result == null || !result.decompileCompleted()) {
            out.addProperty("decompileError", result == null ? "no result" : result.getErrorMessage());
            out.add("decompiledC", JsonNull.INSTANCE);
            out.add("highPcode", new JsonArray());
            out.add("basicBlocks", new JsonArray());
        } else {
            out.addProperty("decompileError", "");
            out.addProperty("decompiledC", result.getDecompiledFunction().getC());
            HighFunction high = result.getHighFunction();
            out.add("highPcode", exportHighPcode(high));
            out.add("basicBlocks", exportBlocks(high));
        }
        out.add("instructionPcode", exportInstructionPcode(function));
        out.add("references", exportReferences(function));
        return out;
    }

    private JsonArray exportLocals(Function function) {
        JsonArray out = new JsonArray();
        for (ghidra.program.model.listing.Variable variable : function.getAllVariables()) {
            JsonObject v = new JsonObject();
            v.addProperty("name", variable.getName());
            v.add("type", dataType(variable.getDataType()));
            v.addProperty("sizeBytes", variable.getLength());
            v.addProperty("storage", variable.getVariableStorage().toString());
            out.add(v);
        }
        return out;
    }

    private JsonArray exportHighPcode(HighFunction high) {
        JsonArray ops = new JsonArray();
        Iterator<PcodeOpAST> iterator = high.getPcodeOps();
        while (iterator.hasNext()) {
            PcodeOpAST op = iterator.next();
            JsonObject item = new JsonObject();
            item.addProperty("sequenceAddress", op.getSeqnum().getTarget().toString());
            item.addProperty("sequenceOrder", op.getSeqnum().getOrder());
            item.addProperty("opcode", op.getMnemonic());
            item.addProperty("opcodeId", op.getOpcode());
            if (op.getOutput() == null) item.add("output", JsonNull.INSTANCE);
            else item.add("output", varnode(op.getOutput()));
            JsonArray inputs = new JsonArray();
            for (int i = 0; i < op.getNumInputs(); i++) inputs.add(varnode(op.getInput(i)));
            item.add("inputs", inputs);
            ops.add(item);
        }
        return ops;
    }

    private JsonArray exportBlocks(HighFunction high) {
        JsonArray blocks = new JsonArray();
        for (PcodeBlockBasic block : high.getBasicBlocks()) {
            JsonObject item = new JsonObject();
            item.addProperty("index", block.getIndex());
            item.addProperty("start", block.getStart().toString());
            item.addProperty("end", block.getStop().toString());
            JsonArray successors = new JsonArray();
            for (int i = 0; i < block.getOutSize(); i++) successors.add(block.getOut(i).getIndex());
            item.add("successors", successors);
            ghidra.program.model.pcode.PcodeOp terminal = block.getLastOp();
            if (terminal != null) {
                item.addProperty("terminalOpcode", terminal.getMnemonic());
                item.addProperty("terminalAddress", terminal.getSeqnum().getTarget().toString());
                JsonArray terminalInputs = new JsonArray();
                for (int i = 0; i < terminal.getNumInputs(); i++) terminalInputs.add(varnode(terminal.getInput(i)));
                item.add("terminalInputs", terminalInputs);
            }
            blocks.add(item);
        }
        return blocks;
    }

    private JsonArray exportInstructionPcode(Function function) {
        JsonArray out = new JsonArray();
        Iterator<Instruction> instructions = currentProgram.getListing().getInstructions(function.getBody(), true);
        while (instructions.hasNext()) {
            Instruction instruction = instructions.next();
            JsonObject row = new JsonObject();
            row.addProperty("address", instruction.getAddress().toString());
            row.addProperty("mnemonic", instruction.getMnemonicString());
            JsonArray operations = new JsonArray();
            for (ghidra.program.model.pcode.PcodeOp op : instruction.getPcode()) {
                JsonObject operation = new JsonObject();
                operation.addProperty("sequenceAddress", op.getSeqnum().getTarget().toString());
                operation.addProperty("sequenceOrder", op.getSeqnum().getOrder());
                operation.addProperty("opcode", op.getMnemonic());
                operation.addProperty("opcodeId", op.getOpcode());
                operation.addProperty("text", op.toString());
                if (op.getOutput() != null) operation.add("output", varnode(op.getOutput()));
                JsonArray inputs = new JsonArray();
                for (int i = 0; i < op.getNumInputs(); i++) inputs.add(varnode(op.getInput(i)));
                operation.add("inputs", inputs);
                operations.add(operation);
            }
            row.add("pcode", operations);
            out.add(row);
        }
        return out;
    }

    private JsonArray exportReferences(Function function) {
        JsonArray out = new JsonArray();
        Iterator<Instruction> instructions = currentProgram.getListing().getInstructions(function.getBody(), true);
        while (instructions.hasNext()) {
            Instruction instruction = instructions.next();
            for (Reference reference : currentProgram.getReferenceManager().getReferencesFrom(instruction.getAddress())) {
                JsonObject item = new JsonObject();
                item.addProperty("from", reference.getFromAddress().toString());
                item.addProperty("to", reference.getToAddress().toString());
                item.addProperty("type", reference.getReferenceType().getName());
                item.addProperty("operandIndex", reference.getOperandIndex());
                out.add(item);
            }
        }
        return out;
    }

    private JsonArray exportDataTypes() {
        JsonArray out = new JsonArray();
        Iterator<DataType> iterator = currentProgram.getDataTypeManager().getAllDataTypes();
        while (iterator.hasNext()) {
            DataType type = iterator.next();
            JsonObject item = dataType(type);
            if (type instanceof Structure) {
                JsonArray fields = new JsonArray();
                Structure structure = (Structure) type;
                for (int i = 0; i < structure.getNumComponents(); i++) {
                    ghidra.program.model.data.DataTypeComponent component = structure.getComponent(i);
                    JsonObject field = new JsonObject();
                    field.addProperty("name", component.getFieldName());
                    field.addProperty("offset", component.getOffset());
                    field.addProperty("sizeBytes", component.getLength());
                    field.add("type", dataType(component.getDataType()));
                    fields.add(field);
                }
                item.add("fields", fields);
            } else if (type instanceof Union) {
                JsonArray fields = new JsonArray();
                Union union = (Union) type;
                for (int i = 0; i < union.getNumComponents(); i++) {
                    ghidra.program.model.data.DataTypeComponent component = union.getComponent(i);
                    JsonObject field = new JsonObject();
                    field.addProperty("name", component.getFieldName());
                    field.addProperty("sizeBytes", component.getLength());
                    field.add("type", dataType(component.getDataType()));
                    fields.add(field);
                }
                item.add("fields", fields);
            } else if (type instanceof Array) {
                Array array = (Array) type;
                item.addProperty("elementCount", array.getNumElements());
                item.add("elementType", dataType(array.getDataType()));
                item.addProperty("elementSizeBytes", array.getElementLength());
            }
            out.add(item);
        }
        return out;
    }

    private JsonObject varnode(Varnode node) {
        JsonObject out = new JsonObject();
        out.addProperty("sizeBytes", node.getSize());
        out.addProperty("space", node.getAddress().getAddressSpace().getName());
        out.addProperty("offsetHex", Long.toUnsignedString(node.getOffset(), 16));
        out.addProperty("constant", node.isConstant());
        out.addProperty("register", node.isRegister());
        out.addProperty("input", node.isInput());
        out.addProperty("unique", node.isUnique());
        if (node.isRegister()) out.addProperty("registerName", node.getAddress().toString());
        if (node.getHigh() != null) {
            out.addProperty("highName", node.getHigh().getName());
            out.add("highType", dataType(node.getHigh().getDataType()));
        } else out.add("highType", JsonNull.INSTANCE);
        return out;
    }

    private JsonObject dataType(DataType type) {
        JsonObject out = new JsonObject();
        out.addProperty("name", type.getName());
        out.addProperty("path", type.getDataTypePath().getPath());
        out.addProperty("kind", type.getClass().getSimpleName());
        out.addProperty("sizeBytes", type.getLength());
        out.addProperty("alignmentBytes", type.getAlignment());
        out.addProperty("description", type.getDescription());
        return out;
    }
}
