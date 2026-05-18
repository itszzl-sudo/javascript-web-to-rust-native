use crate::analyzer::{Analyzer, AnalysisResult};
use crate::ast::Program;
use crate::codegen::CodeGen;
use crate::error::Result;
use crate::ir_gen::IrGenerator;
use crate::swc_parser::SwcParser;

pub struct Compiler {
    parser: SwcParser,
    analyzer: Analyzer,
    codegen: CodeGen,
    ir_gen: IrGenerator,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            parser: SwcParser::new(),
            analyzer: Analyzer::new(),
            codegen: CodeGen::new(),
            ir_gen: IrGenerator::new(),
        }
    }

    pub fn compile(&mut self, source: &str) -> Result<CompileResult> {
        let module = self.parser.parse(source).map_err(|e: crate::swc_parser::SwcParseError| crate::error::Error::ParseError {
            location: "line 1".to_string(),
            message: e.to_string(),
        })?;
        
        let body = crate::swc_parser::swc_module_to_ast(module);
        
        let program = Program {
            source_type: crate::ast::SourceType::Module,
            body,
            loc: crate::ast::SourceLocation::default(),
        };
        
        let analysis = self.analyzer.analyze(&program)?;
        let code = self.codegen.generate(&program)?;

        Ok(CompileResult {
            ast: program,
            analysis,
            code,
        })
    }
    
    pub fn compile_to_ir(&mut self, source: &str) -> Result<cranelift_compiler::Program> {
        let module = self.parser.parse(source).map_err(|e: crate::swc_parser::SwcParseError| crate::error::Error::ParseError {
            location: "line 1".to_string(),
            message: e.to_string(),
        })?;
        
        let body = crate::swc_parser::swc_module_to_ast(module);
        
        let program = Program {
            source_type: crate::ast::SourceType::Module,
            body,
            loc: crate::ast::SourceLocation::default(),
        };
        
        self.ir_gen.generate(&program)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CompileResult {
    pub ast: Program,
    pub analysis: AnalysisResult,
    pub code: String,
}
