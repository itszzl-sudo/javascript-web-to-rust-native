//! 框架检测模块
//!
//! 根据编译后的 JavaScript 代码特征检测使用的框架

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Framework {
    Vue3,
    Svelte,
    Preact,
    Solid,
    React,
    Angular,
    Qwik,
    Lit,
    Vanilla,
}

impl Framework {
    pub fn name(&self) -> &'static str {
        match self {
            Framework::Vue3 => "Vue 3",
            Framework::Svelte => "Svelte",
            Framework::Preact => "Preact",
            Framework::Solid => "SolidJS",
            Framework::React => "React",
            Framework::Angular => "Angular",
            Framework::Qwik => "Qwik",
            Framework::Lit => "Lit",
            Framework::Vanilla => "Vanilla JS",
        }
    }
    
    pub fn runtime_bindings(&self) -> &'static [&'static str] {
        match self {
            Framework::Vue3 => &[
                "createElementBlock",
                "createVNode",
                "createBaseVNode",
                "createTextVNode",
                "toDisplayString",
                "normalizeClass",
                "normalizeStyle",
                "openBlock",
                "setBlockTracking",
            ],
            Framework::Svelte => &[
                "svelte_element",
                "svelte_text",
                "svelte_space",
                "svelte_insert",
                "svelte_append",
                "svelte_detach",
                "svelte_attr",
                "svelte_set_data",
                "svelte_listen",
            ],
            Framework::Preact => &[
                "preact_h",
                "preact_fragment",
                "preact_createElement",
                "preact_render",
            ],
            Framework::Solid => &[
                "solid_createSignal",
                "solid_createEffect",
                "solid_createMemo",
                "solid_insert",
                "solid_template",
            ],
            Framework::React => &[
                "React.createElement",
                "useState",
                "useEffect",
                "useRef",
                "useMemo",
                "useCallback",
            ],
            Framework::Angular => &[
                "ɵɵelementStart",
                "ɵɵelementEnd",
                "ɵɵtext",
                "ɵɵadvance",
                "ɵɵproperty",
                "ɵɵlistener",
            ],
            Framework::Qwik => &[
                "qrl",
                "useSignal",
                "useStore",
                "useEffect$",
            ],
            Framework::Lit => &[
                "html",
                "render",
                "unsafeHTML",
            ],
            Framework::Vanilla => &[],
        }
    }
    
    pub fn priority(&self) -> u8 {
        match self {
            Framework::Vue3 | Framework::Svelte => 0,
            Framework::Preact | Framework::Solid => 1,
            Framework::Lit => 2,
            Framework::React => 3,
            Framework::Angular | Framework::Qwik => 4,
            Framework::Vanilla => 5,
        }
    }
}

#[derive(Debug)]
pub struct FrameworkDetectionResult {
    pub primary: Framework,
    pub detected: Vec<Framework>,
    pub confidence: f32,
}

pub fn detect_framework(js_code: &str) -> FrameworkDetectionResult {
    let patterns = vec![
        (Framework::Vue3, detect_vue3_patterns(js_code)),
        (Framework::Svelte, detect_svelte_patterns(js_code)),
        (Framework::Preact, detect_preact_patterns(js_code)),
        (Framework::Solid, detect_solid_patterns(js_code)),
        (Framework::React, detect_react_patterns(js_code)),
        (Framework::Angular, detect_angular_patterns(js_code)),
        (Framework::Qwik, detect_qwik_patterns(js_code)),
        (Framework::Lit, detect_lit_patterns(js_code)),
    ];
    
    let detected: Vec<(Framework, usize)> = patterns
        .into_iter()
        .filter(|(_, count)| *count > 0)
        .collect();
    
    if detected.is_empty() {
        return FrameworkDetectionResult {
            primary: Framework::Vanilla,
            detected: vec![],
            confidence: 1.0,
        };
    }
    
    let max_count = detected.iter().map(|(_, c)| *c).max().unwrap_or(0);
    let mut sorted = detected;
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    let primary = sorted[0].0;
    let confidence = sorted[0].1 as f32 / (sorted[0].1 + sorted.get(1).map(|(_, c)| *c).unwrap_or(0) + 1) as f32;
    
    FrameworkDetectionResult {
        primary,
        detected: sorted.into_iter().map(|(f, _)| f).collect(),
        confidence,
    }
}

fn detect_vue3_patterns(code: &str) -> usize {
    let patterns = [
        "createElementBlock",
        "createVNode",
        "createBaseVNode",
        "createTextVNode",
        "toDisplayString",
        "openBlock",
        "normalizeClass",
        "normalizeStyle",
        "_createElementBlock",
        "_createVNode",
    ];
    
    patterns.iter().map(|p| code.matches(p).count()).sum()
}

fn detect_svelte_patterns(code: &str) -> usize {
    let patterns = [
        "create_fragment",
        "svelte_element",
        "svelte_text",
        "svelte_insert",
        "svelte_append",
        "svelte_detach",
        "function c()",
        "function m(",
        "function p(",
        "function d(",
    ];
    
    patterns.iter().map(|p| code.matches(p).count()).sum()
}

fn detect_preact_patterns(code: &str) -> usize {
    let patterns = [
        "preact",
        "h(",
        "h(",
        "Fragment",
        "__v",
        "__b",
        "__r",
        "diffProps",
    ];
    
    let base_count = patterns.iter().map(|p| code.matches(p).count()).sum::<usize>();
    
    if code.contains("React.createElement") && code.contains("preact") {
        base_count + 10
    } else {
        base_count
    }
}

fn detect_solid_patterns(code: &str) -> usize {
    let patterns = [
        "createSignal",
        "createEffect",
        "createMemo",
        "createResource",
        "createContext",
        "useContext",
        "onMount",
        "onCleanup",
        "untrack",
        "batch",
        "solid-js",
    ];
    
    patterns.iter().map(|p| code.matches(p).count()).sum()
}

fn detect_react_patterns(code: &str) -> usize {
    let patterns = [
        "React.createElement",
        "useState",
        "useEffect",
        "useRef",
        "useMemo",
        "useCallback",
        "useContext",
        "useReducer",
        "React.Component",
        "_jsx",
        "_jsxs",
    ];
    
    patterns.iter().map(|p| code.matches(p).count()).sum()
}

fn detect_angular_patterns(code: &str) -> usize {
    let patterns = [
        "ɵɵelementStart",
        "ɵɵelementEnd",
        "ɵɵtext",
        "ɵɵadvance",
        "ɵɵproperty",
        "ɵɵlistener",
        "ɵɵattribute",
        "ɵɵclassProp",
        "ɵɵstyleProp",
        "ɵɵreference",
        "ɵɵtemplate",
        "ɵɵpipe",
    ];
    
    patterns.iter().map(|p| code.matches(p).count()).sum()
}

fn detect_qwik_patterns(code: &str) -> usize {
    let patterns = [
        "qrl",
        "useSignal",
        "useStore",
        "useEffect$",
        "useVisibleTask$",
        "useResource$",
        "useContext",
        "$(",
        "useOn",
        "useOnDocument",
    ];
    
    patterns.iter().map(|p| code.matches(p).count()).sum()
}

fn detect_lit_patterns(code: &str) -> usize {
    let patterns = [
        "html`",
        "css`",
        "render",
        "unsafeHTML",
        "repeat",
        "map",
        "ifDefined",
        "classMap",
        "styleMap",
        "LitElement",
        "@customElement",
    ];
    
    patterns.iter().map(|p| code.matches(p).count()).sum()
}

pub fn get_binding_module(framework: Framework) -> &'static str {
    match framework {
        Framework::Vue3 => "vue",
        Framework::Svelte => "svelte",
        Framework::Preact => "preact",
        Framework::Solid => "solid",
        Framework::React => "react",
        Framework::Angular => "angular",
        Framework::Qwik => "qwik",
        Framework::Lit => "lit",
        Framework::Vanilla => "dom",
    }
}

pub fn get_register_function(framework: Framework) -> String {
    format!("register_{}_bindings", get_binding_module(framework))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_vue3() {
        let code = r#"
            function render(_ctx, _cache) {
                return openBlock(), createElementBlock("div", null, [
                    createBaseVNode("h1", null, toDisplayString(_ctx.title), 1)
                ]);
            }
        "#;
        
        let result = detect_framework(code);
        assert_eq!(result.primary, Framework::Vue3);
    }
    
    #[test]
    fn test_detect_svelte() {
        let code = r#"
            function create_fragment(ctx) {
                let button;
                function c() {
                    button = element("button");
                }
                function m(target, anchor) {
                    insert(target, button, anchor);
                }
                return { c, m, p: noop, d };
            }
        "#;
        
        let result = detect_framework(code);
        assert_eq!(result.primary, Framework::Svelte);
    }
    
    #[test]
    fn test_detect_react() {
        let code = r#"
            function App() {
                const [count, setCount] = useState(0);
                return React.createElement("div", null, count);
            }
        "#;
        
        let result = detect_framework(code);
        assert_eq!(result.primary, Framework::React);
    }
    
    #[test]
    fn test_detect_solid() {
        let code = r#"
            function Counter() {
                const [count, setCount] = createSignal(0);
                createEffect(() => console.log(count()));
                return count;
            }
        "#;
        
        let result = detect_framework(code);
        assert_eq!(result.primary, Framework::Solid);
    }
    
    #[test]
    fn test_detect_vanilla() {
        let code = r#"
            const button = document.createElement("button");
            button.addEventListener("click", () => {
                console.log("clicked");
            });
        "#;
        
        let result = detect_framework(code);
        assert_eq!(result.primary, Framework::Vanilla);
    }
}
