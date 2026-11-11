use serde::{Deserialize, Serialize};
use std::{
    fmt, fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_TEMP: AtomicUsize = AtomicUsize::new(0);
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GhidraExport {
    pub schema_version: u32,
    pub ghidra_version: String,
    pub program: ProgramInfo,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub scope_complete: bool,
    #[serde(default)]
    pub program_function_count: u32,
    #[serde(default)]
    pub function_count: u32,
    #[serde(default)]
    pub functions: Vec<GhidraFunction>,
    #[serde(default)]
    pub data_types: Vec<DataTypeInfo>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramInfo {
    pub name: String,
    pub executable_path: String,
    pub language_id: String,
    pub compiler_spec_id: String,
    pub big_endian: bool,
    pub pointer_size: u32,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GhidraFunction {
    pub name: String,
    pub entry: String,
    pub body_bytes: u64,
    pub calling_convention: String,
    pub is_external: bool,
    pub is_thunk: bool,
    pub no_return: bool,
    pub return_type: DataTypeInfo,
    #[serde(default)]
    pub parameters: Vec<VariableInfo>,
    #[serde(default)]
    pub locals: Vec<VariableInfo>,
    #[serde(default)]
    pub decompile_completed: bool,
    #[serde(default)]
    pub decompile_error: String,
    pub decompiled_c: Option<String>,
    #[serde(default)]
    pub high_pcode: Vec<PcodeOp>,
    #[serde(default)]
    pub basic_blocks: Vec<BasicBlock>,
    #[serde(default)]
    pub instruction_pcode: Vec<InstructionPcode>,
    #[serde(default)]
    pub references: Vec<ReferenceInfo>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: DataTypeInfo,
    pub size_bytes: u32,
    pub storage: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataTypeInfo {
    pub name: String,
    pub path: String,
    pub kind: String,
    pub size_bytes: i64,
    pub alignment_bytes: Option<i32>,
    pub description: String,
    #[serde(default)]
    pub fields: Vec<FieldInfo>,
    pub element_count: Option<u64>,
    pub element_type: Option<Box<DataTypeInfo>>,
    pub element_size_bytes: Option<u32>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo {
    pub name: Option<String>,
    pub offset: Option<u64>,
    pub size_bytes: u32,
    #[serde(rename = "type")]
    pub data_type: DataTypeInfo,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PcodeOp {
    pub sequence_address: String,
    pub sequence_order: u32,
    pub opcode: String,
    pub opcode_id: u32,
    pub output: Option<Varnode>,
    #[serde(default)]
    pub inputs: Vec<Varnode>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Varnode {
    pub size_bytes: u32,
    pub space: String,
    pub offset_hex: String,
    pub constant: bool,
    pub register: bool,
    pub input: bool,
    pub unique: bool,
    pub register_name: Option<String>,
    pub high_name: Option<String>,
    pub high_type: Option<DataTypeInfo>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicBlock {
    pub index: u32,
    pub start: String,
    pub end: String,
    #[serde(default)]
    pub successors: Vec<u32>,
    pub terminal_opcode: Option<String>,
    pub terminal_address: Option<String>,
    #[serde(default)]
    pub terminal_inputs: Vec<Varnode>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionPcode {
    pub address: String,
    pub mnemonic: String,
    #[serde(default)]
    pub pcode: Vec<RawPcode>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct RawPcode {
    pub opcode: String,
    pub text: String,
    pub sequence_address: Option<String>,
    pub sequence_order: Option<u32>,
    pub opcode_id: Option<u32>,
    pub output: Option<Varnode>,
    #[serde(default)]
    pub inputs: Vec<Varnode>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceInfo {
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub operand_index: i32,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FunctionStatus {
    Safe,
    UnsafeBoundary,
    Unresolved,
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionFinding {
    pub name: String,
    pub entry: String,
    pub status: FunctionStatus,
    pub evidence: Vec<String>,
    pub reasons: Vec<String>,
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceMapping {
    pub source_line_start: u32,
    pub source_line_end: u32,
    pub binary_entry: String,
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionReport {
    pub schema_version: u32,
    pub ghidra_version: String,
    pub program: String,
    pub complete: bool,
    pub scope_complete: bool,
    pub static_safety_verified: bool,
    pub generated_source_compiles: bool,
    pub behavioral_equivalence: String,
    pub functions: Vec<FunctionFinding>,
    pub source_mappings: Vec<SourceMapping>,
    pub limitations: Vec<String>,
}
#[derive(Clone, Debug)]
pub struct LoweredExport {
    pub source: Option<String>,
    pub report: ConversionReport,
}
#[derive(Debug)]
pub struct GhidraError(pub String);
impl fmt::Display for GhidraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for GhidraError {}

pub fn load_export(path: &Path) -> Result<GhidraExport, GhidraError> {
    let bytes =
        fs::read(path).map_err(|e| GhidraError(format!("cannot read Ghidra export: {e}")))?;
    let export: GhidraExport = serde_json::from_slice(&bytes)
        .map_err(|e| GhidraError(format!("invalid Ghidra export JSON: {e}")))?;
    if export.schema_version != 1 {
        return Err(GhidraError(format!(
            "unsupported Ghidra export schema {}; expected 1",
            export.schema_version
        )));
    }
    Ok(export)
}

pub fn lower_export_json(json: &str) -> Result<LoweredExport, GhidraError> {
    let export: GhidraExport = serde_json::from_str(json)
        .map_err(|e| GhidraError(format!("invalid Ghidra export JSON: {e}")))?;
    if export.schema_version != 1 {
        return Err(GhidraError(format!(
            "unsupported Ghidra export schema {}; expected 1",
            export.schema_version
        )));
    }
    lower_export(&export)
}

pub fn lower_file(input: &Path, output: &Path) -> Result<ConversionReport, GhidraError> {
    if output.extension().and_then(|e| e.to_str()) != Some("as") {
        return Err(GhidraError("Aster output must have `.as` extension".into()));
    }
    let export = load_export(input)?;
    let mut lowered = lower_export(&export)?;
    if let Some(source) = &lowered.source {
        let temp = temp_path("native");
        crate::backend::build(source, &temp).map_err(|e| {
            GhidraError(format!(
                "generated Aster did not pass safety verification/native compilation: {e}"
            ))
        })?;
        let _ = fs::remove_file(temp);
        lowered.report.static_safety_verified = true;
        lowered.report.generated_source_compiles = true;
        fs::write(output, source)
            .map_err(|e| GhidraError(format!("cannot write Aster source: {e}")))?;
    }
    fs::write(
        output.with_extension("ghidra-report.json"),
        serde_json::to_vec_pretty(&lowered.report)
            .map_err(|e| GhidraError(format!("cannot serialize conversion report: {e}")))?,
    )
    .map_err(|e| GhidraError(format!("cannot write Ghidra conversion report: {e}")))?;
    Ok(lowered.report)
}

fn lower_export(export: &GhidraExport) -> Result<LoweredExport, GhidraError> {
    if export.functions.is_empty() {
        return Err(GhidraError("Ghidra export contains no functions".into()));
    }
    let mut findings = Vec::new();
    let mut main_result = None;
    let mut main_entry = None;
    for function in &export.functions {
        let (status, evidence, reasons, value) = analyze_function(function);
        if function.name == "main" && status == FunctionStatus::Safe {
            main_result = value;
            main_entry = Some(function.entry.clone());
        }
        findings.push(FunctionFinding {
            name: function.name.clone(),
            entry: function.entry.clone(),
            status,
            evidence,
            reasons,
        });
    }
    let source = main_result.map(|value| format!("fn main() -> i32 {{\n    return {value};\n}}\n"));
    if source.is_none() {
        for finding in &mut findings {
            if finding.name == "main" && finding.status == FunctionStatus::Safe {
                finding.status = FunctionStatus::Unresolved;
            }
        }
    }
    let scope_complete = export.scope_complete
        && export.program_function_count as usize == export.functions.len()
        && export.function_count as usize == export.functions.len();
    let complete = source.is_some()
        && scope_complete
        && findings.iter().all(|f| f.status == FunctionStatus::Safe);
    let source_mappings = main_entry
        .map(|entry| {
            vec![SourceMapping {
                source_line_start: 1,
                source_line_end: 3,
                binary_entry: entry,
            }]
        })
        .unwrap_or_default();
    let report=ConversionReport{
        schema_version:1,ghidra_version:export.ghidra_version.clone(),program:export.program.name.clone(),complete,scope_complete,
        static_safety_verified:false,generated_source_compiles:false,behavioral_equivalence:"not_run; compilation is not behavioral equivalence".into(),functions:findings,source_mappings,
        limitations:vec![
            "Only a single P-code-proven constant-return `main` is lowered; all other functions must be safe and understood for complete conversion.".into(),
            "Selection- or function-filtered exports cannot establish complete program coverage.".into(),
            "Ghidra recovered types are evidence, not ground truth; only signed 32-bit integer return type is accepted.".into(),
            "No ownership, pointer, heap, struct, floating-point, call, or control-flow reconstruction is performed by this conservative first lowering.".into(),
        ],
    };
    Ok(LoweredExport { source, report })
}

fn analyze_function(
    function: &GhidraFunction,
) -> (FunctionStatus, Vec<String>, Vec<String>, Option<i32>) {
    let mut evidence = vec![format!("entry {}", function.entry)];
    let reasons = Vec::new();
    if function.is_external {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["external function has no local body to recover".into()],
            None,
        );
    }
    if !function.decompile_completed {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec![format!(
                "decompilation failed: {}",
                function.decompile_error
            )],
            None,
        );
    }
    if function.no_return || function.is_thunk {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["no-return and thunk functions are outside the supported lowering".into()],
            None,
        );
    }
    if function.name != "main" {
        if let Some(op) = function.high_pcode.iter().find(|op| {
            matches!(
                op.opcode.as_str(),
                "LOAD" | "STORE" | "PTRADD" | "PTRSUB" | "CALLIND"
            )
        }) {
            return (
                FunctionStatus::UnsafeBoundary,
                evidence,
                vec![format!(
                    "operation `{}` requires unresolved alias/lifetime evidence",
                    op.opcode
                )],
                None,
            );
        }
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["only the program entry `main` is emitted by this lowering".into()],
            None,
        );
    }
    if !function.parameters.is_empty() {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["parameters are outside the supported constant-main subset".into()],
            None,
        );
    }
    if function.return_type.size_bytes != 4
        || function.return_type.kind != "IntegerDataType"
        || !matches!(function.return_type.name.as_str(), "int" | "int32_t")
    {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec![format!(
                "return type is not proven signed 32-bit integer: {} ({}, {} bytes)",
                function.return_type.name,
                function.return_type.kind,
                function.return_type.size_bytes
            )],
            None,
        );
    }
    if function.basic_blocks.len() != 1
        || function
            .basic_blocks
            .first()
            .is_none_or(|b| !b.successors.is_empty())
    {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["control flow is not a single terminal block".into()],
            None,
        );
    }
    let mut copies = Vec::new();
    let mut returns = Vec::new();
    for op in &function.high_pcode {
        match op.opcode.as_str() {
            "COPY" => copies.push(op),
            "RETURN" => returns.push(op),
            "LOAD" | "STORE" | "PTRADD" | "PTRSUB" | "CALLIND" => {
                return (
                    FunctionStatus::UnsafeBoundary,
                    evidence,
                    vec![format!(
                        "memory or indirect operation `{}` requires unresolved alias/lifetime evidence",
                        op.opcode
                    )],
                    None,
                );
            }
            _ => {
                return (
                    FunctionStatus::Unresolved,
                    evidence,
                    vec![format!("unsupported P-code operation `{}`", op.opcode)],
                    None,
                );
            }
        }
    }
    if copies.len() != 1 || returns.len() != 1 {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["expected exactly one COPY and one RETURN operation".into()],
            None,
        );
    }
    let copy = copies[0];
    let Some(output) = copy.output.as_ref() else {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["COPY has no output varnode".into()],
            None,
        );
    };
    if copy.inputs.len() != 1 || !output.register {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["COPY does not assign one register from one value".into()],
            None,
        );
    }
    let value = &copy.inputs[0];
    if !value.constant || value.size_bytes != 4 || value.space != "const" {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["return value is not a 32-bit P-code constant".into()],
            None,
        );
    }
    let ret = returns[0];
    let Some(ret_value) = ret.inputs.last() else {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["RETURN does not expose a return-value varnode".into()],
            None,
        );
    };
    if !same_storage(output, ret_value) {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec!["RETURN value is not the constant-defined output register".into()],
            None,
        );
    }
    let block = &function.basic_blocks[0];
    if !address_in_block(&copy.sequence_address, block)
        || !address_in_block(&ret.sequence_address, block)
        || !sequence_before(copy, ret)
    {
        return (
            FunctionStatus::Unresolved,
            evidence,
            vec![
                "P-code COPY is not proven to execute before RETURN in the recovered block".into(),
            ],
            None,
        );
    }
    let raw = match u32::from_str_radix(&value.offset_hex, 16) {
        Ok(x) => x,
        Err(_) => {
            return (
                FunctionStatus::Unresolved,
                evidence,
                vec!["constant varnode offset is malformed".into()],
                None,
            );
        }
    };
    let result = raw as i32;
    evidence.push(format!(
        "{}-byte signed integer return recovered from P-code constant 0x{} at {}",
        value.size_bytes, value.offset_hex, copy.sequence_address
    ));
    evidence.push(format!(
        "RETURN at {} consumes the COPY output register",
        ret.sequence_address
    ));
    (FunctionStatus::Safe, evidence, reasons, Some(result))
}
fn same_storage(a: &Varnode, b: &Varnode) -> bool {
    a.space == b.space && a.offset_hex == b.offset_hex && a.size_bytes == b.size_bytes
}
fn address_value(address: &str) -> Option<u128> {
    u128::from_str_radix(address.rsplit(':').next()?.trim_start_matches("0x"), 16).ok()
}
fn address_in_block(address: &str, block: &BasicBlock) -> bool {
    match (
        address_value(address),
        address_value(&block.start),
        address_value(&block.end),
    ) {
        (Some(value), Some(start), Some(end)) => (start..=end).contains(&value),
        _ => false,
    }
}
fn sequence_before(a: &PcodeOp, b: &PcodeOp) -> bool {
    match (
        address_value(&a.sequence_address),
        address_value(&b.sequence_address),
    ) {
        (Some(a_address), Some(b_address)) => {
            (a_address, a.sequence_order) < (b_address, b.sequence_order)
        }
        _ => false,
    }
}
fn temp_path(ext: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "aster-ghidra-{}-{}.{}",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed),
        ext
    ))
}

pub fn write_report(path: &Path, report: &ConversionReport) -> Result<(), GhidraError> {
    fs::write(
        path,
        serde_json::to_vec_pretty(report)
            .map_err(|e| GhidraError(format!("cannot serialize report: {e}")))?,
    )
    .map_err(|e| GhidraError(format!("cannot write report: {e}")))
}

#[cfg(test)]
mod tests {
    use super::{FunctionStatus, lower_export_json};

    const SAFE_EXPORT: &str = r#"{
      "schemaVersion":1,"ghidraVersion":"12.1.4","scope":"program","scopeComplete":true,"programFunctionCount":1,"functionCount":1,
      "program":{"name":"tiny","executablePath":"/tmp/tiny","languageId":"x86:LE:64:default","compilerSpecId":"gcc","bigEndian":false,"pointerSize":8},
      "functions":[{"name":"main","entry":"00401000","bodyBytes":6,"callingConvention":"unknown","isExternal":false,"isThunk":false,"noReturn":false,
        "returnType":{"name":"int","path":"/int","kind":"IntegerDataType","sizeBytes":4,"alignmentBytes":4,"description":"signed int"},
        "parameters":[],"locals":[],"decompileCompleted":true,"decompileError":"","decompiledC":"int main(void){return 7;}",
        "highPcode":[
          {"sequenceAddress":"00401005","sequenceOrder":0,"opcode":"RETURN","opcodeId":10,"inputs":[{"sizeBytes":8,"space":"const","offsetHex":"0","constant":true,"register":false,"input":false,"unique":false},{"sizeBytes":4,"space":"register","offsetHex":"0","constant":false,"register":true,"input":false,"unique":false}]},
          {"sequenceAddress":"00401000","sequenceOrder":0,"opcode":"COPY","opcodeId":1,"output":{"sizeBytes":4,"space":"register","offsetHex":"0","constant":false,"register":true,"input":false,"unique":false},"inputs":[{"sizeBytes":4,"space":"const","offsetHex":"7","constant":true,"register":false,"input":false,"unique":false}]}
        ],"basicBlocks":[{"index":0,"start":"00401000","end":"00401005","successors":[]}],"instructionPcode":[],"references":[]}],"dataTypes":[]
    }"#;

    #[test]
    fn lowers_only_pcode_proven_constant_main() {
        let lowered = lower_export_json(SAFE_EXPORT).unwrap();
        assert_eq!(
            lowered.source.unwrap(),
            "fn main() -> i32 {\n    return 7;\n}\n"
        );
        assert!(lowered.report.complete);
        assert_eq!(lowered.report.functions[0].status, FunctionStatus::Safe);
    }

    #[test]
    fn filtered_exports_cannot_be_reported_as_complete() {
        let json = SAFE_EXPORT.replace("\"scopeComplete\":true", "\"scopeComplete\":false");
        let result = lower_export_json(&json).unwrap();
        assert!(result.source.is_some());
        assert!(!result.report.complete);
        assert!(!result.report.scope_complete);
    }

    #[test]
    fn rejects_a_copy_not_proven_to_precede_return() {
        let json = SAFE_EXPORT.replace("00401000", "00401006");
        let result = lower_export_json(&json).unwrap();
        assert!(result.source.is_none());
        assert_eq!(
            result.report.functions[0].status,
            FunctionStatus::Unresolved
        );
    }

    #[test]
    fn only_main_is_emitted_even_when_a_helper_has_the_same_safe_shape() {
        let json = SAFE_EXPORT.replace("\"name\":\"main\"", "\"name\":\"helper\"");
        let result = lower_export_json(&json).unwrap();
        assert!(result.source.is_none());
        assert_eq!(
            result.report.functions[0].status,
            FunctionStatus::Unresolved
        );
    }

    #[test]
    fn memory_ops_are_marked_unsafe_and_never_lowered_as_safe() {
        let json = SAFE_EXPORT.replace("\"opcode\":\"COPY\"", "\"opcode\":\"LOAD\"");
        let result = lower_export_json(&json).unwrap();
        assert!(!result.report.complete);
        assert!(result.source.is_none());
        assert_eq!(
            result.report.functions[0].status,
            FunctionStatus::UnsafeBoundary
        );
    }
}
