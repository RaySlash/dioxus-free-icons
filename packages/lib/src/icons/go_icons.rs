use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMirror;
impl IconShape for GoMirror {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.75 1.75a.75.75 0 00-1.5 0v.5a.75.75 0 001.5 0v-.5zM8 4a.75.75 0 01.75.75v.5a.75.75 0 01-1.5 0v-.5A.75.75 0 018 4zm.75 3.75a.75.75 0 00-1.5 0v.5a.75.75 0 001.5 0v-.5zM8 10a.75.75 0 01.75.75v.5a.75.75 0 01-1.5 0v-.5A.75.75 0 018 10zm0 3a.75.75 0 01.75.75v.5a.75.75 0 01-1.5 0v-.5A.75.75 0 018 13zm7.547-9.939A.75.75 0 0116 3.75v8.5a.75.75 0 01-1.265.545l-4.5-4.25a.75.75 0 010-1.09l4.5-4.25a.75.75 0 01.812-.144zM11.842 8l2.658 2.51V5.49L11.842 8zM0 12.25a.75.75 0 001.265.545l4.5-4.25a.75.75 0 000-1.09l-4.5-4.25A.75.75 0 000 3.75v8.5zm1.5-6.76L4.158 8 1.5 10.51V5.49z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodescan;
impl IconShape for GoCodescan {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.47 4.97a.75.75 0 000 1.06L9.94 7.5 8.47 8.97a.75.75 0 101.06 1.06l2-2a.75.75 0 000-1.06l-2-2a.75.75 0 00-1.06 0zM6.53 6.03a.75.75 0 00-1.06-1.06l-2 2a.75.75 0 000 1.06l2 2a.75.75 0 101.06-1.06L5.06 7.5l1.47-1.47z",
            }
            path {
                d: "M12.246 13.307a7.5 7.5 0 111.06-1.06l2.474 2.473a.75.75 0 11-1.06 1.06l-2.474-2.473zM1.5 7.5a6 6 0 1110.386 4.094.75.75 0 00-.292.293A6 6 0 011.5 7.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoX;
impl IconShape for GoX {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLinkExternal;
impl IconShape for GoLinkExternal {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.604 1h4.146a.25.25 0 01.25.25v4.146a.25.25 0 01-.427.177L13.03 4.03 9.28 7.78a.75.75 0 01-1.06-1.06l3.75-3.75-1.543-1.543A.25.25 0 0110.604 1zM3.75 2A1.75 1.75 0 002 3.75v8.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 12.25v-3.5a.75.75 0 00-1.5 0v3.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-8.5a.25.25 0 01.25-.25h3.5a.75.75 0 000-1.5h-3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoListUnordered;
impl IconShape for GoListUnordered {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 4a1 1 0 100-2 1 1 0 000 2zm3.75-1.5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zM3 8a1 1 0 11-2 0 1 1 0 012 0zm-1 6a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHorizontalRule;
impl IconShape for GoHorizontalRule {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 7.75A.75.75 0 01.75 7h14.5a.75.75 0 010 1.5H.75A.75.75 0 010 7.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedRocket;
impl IconShape for GoFeedRocket {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm3.031-12a4.38 4.38 0 00-3.097 1.283l-.23.229c-.156.157-.308.32-.452.49H5.65a.876.876 0 00-.746.417l-.856 1.388a.375.375 0 00.21.556l1.552.477 1.35 1.35.478 1.553a.375.375 0 00.555.21l1.389-.855a.876.876 0 00.416-.746V8.747c.17-.144.333-.295.49-.452l.23-.23A4.38 4.38 0 0012 4.969v-.093A.876.876 0 0011.124 4h-.093zm-5.107 7.144a.81.81 0 01-.188.263c-.394.394-1.258.563-1.62.619a.124.124 0 01-.143-.143c.056-.362.225-1.226.62-1.62a.808.808 0 011.33.881z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodeOfConduct;
impl IconShape for GoCodeOfConduct {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.048 2.241c.964-.709 2.079-1.238 3.325-1.241a4.613 4.613 0 013.282 1.355c.41.408.757.86.996 1.428.238.568.348 1.206.347 1.968 0 2.193-1.505 4.254-3.081 5.862-1.496 1.526-3.213 2.796-4.249 3.563l-.22.163a.75.75 0 01-.895 0l-.221-.163c-1.036-.767-2.753-2.037-4.249-3.563C1.51 10.008.007 7.952.002 5.762a4.614 4.614 0 011.353-3.407C3.123.585 6.223.537 8.048 2.24zm-1.153.983c-.81.78-1.546 1.669-2.166 2.417-.184.222-.358.432-.52.623a.75.75 0 00.04 1.016c.35.35.697.697 1.043 1.047.866.875 2.292.914 3.185.032.264-.26.534-.528.802-.797.694-.694 1.8-.701 2.474-.03L12.92 8.7l.283.284c-.244.334-.515.666-.81.995l-1.384-1.28A.75.75 0 109.99 9.802l1.357 1.252c-.325.31-.656.606-.984.887l-1.48-1.366a.75.75 0 10-1.018 1.102L9.191 12.9c-.433.34-.838.643-1.191.905-1.04-.773-2.537-1.907-3.846-3.242C2.611 8.99 1.502 7.306 1.502 5.75a3.114 3.114 0 01.913-2.335c1.159-1.158 3.23-1.224 4.48-.191zm7.112 4.442c.313-.65.491-1.293.491-1.916v-.001c0-.614-.088-1.045-.23-1.385-.143-.339-.357-.633-.673-.949a3.113 3.113 0 00-2.218-.915c-1.092.003-2.165.627-3.226 1.602-.823.755-1.554 1.637-2.228 2.45l-.127.154.562.566a.756.756 0 001.066.02l.794-.79c1.258-1.258 3.312-1.31 4.594-.032.396.394.792.791 1.173 1.173l.022.023z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoPull;
impl IconShape for GoRepoPull {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 8V6H7V4h6V2l3 3-3 3zM4 2H3v1h1V2zm7 5h1v6c0 .55-.45 1-1 1H6v2l-1.5-1.5L3 16v-2H1c-.55 0-1-.45-1-1V1c0-.55.45-1 1-1h10c.55 0 1 .45 1 1v2h-1V1H2v9h9V7zm0 4H1v2h2v-1h3v1h5v-2zM4 6H3v1h1V6zm0-2H3v1h1V4zM3 9h1V8H3v1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitPullRequestClosed;
impl IconShape for GoGitPullRequestClosed {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.72 1.227a.75.75 0 011.06 0l.97.97.97-.97a.75.75 0 111.06 1.061l-.97.97.97.97a.75.75 0 01-1.06 1.06l-.97-.97-.97.97a.75.75 0 11-1.06-1.06l.97-.97-.97-.97a.75.75 0 010-1.06zM12.75 6.5a.75.75 0 00-.75.75v3.378a2.251 2.251 0 101.5 0V7.25a.75.75 0 00-.75-.75zm0 5.5a.75.75 0 100 1.5.75.75 0 000-1.5zM2.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.25 1a2.25 2.25 0 00-.75 4.372v5.256a2.251 2.251 0 101.5 0V5.372A2.25 2.25 0 003.25 1zm0 11a.75.75 0 100 1.5.75.75 0 000-1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDatabase;
impl IconShape for GoDatabase {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.5 3.5c0-.133.058-.318.282-.55.227-.237.592-.484 1.1-.708C4.899 1.795 6.354 1.5 8 1.5c1.647 0 3.102.295 4.117.742.51.224.874.47 1.101.707.224.233.282.418.282.551 0 .133-.058.318-.282.55-.227.237-.592.484-1.1.708C11.101 5.205 9.646 5.5 8 5.5c-1.647 0-3.102-.295-4.117-.742-.51-.224-.874-.47-1.101-.707-.224-.233-.282-.418-.282-.551zM1 3.5c0-.626.292-1.165.7-1.59.406-.422.956-.767 1.579-1.041C4.525.32 6.195 0 8 0c1.805 0 3.475.32 4.722.869.622.274 1.172.62 1.578 1.04.408.426.7.965.7 1.591v9c0 .626-.292 1.165-.7 1.59-.406.422-.956.767-1.579 1.041C11.476 15.68 9.806 16 8 16c-1.805 0-3.475-.32-4.721-.869-.623-.274-1.173-.62-1.579-1.04-.408-.426-.7-.965-.7-1.591v-9zM2.5 8V5.724c.241.15.503.286.779.407C4.525 6.68 6.195 7 8 7c1.805 0 3.475-.32 4.722-.869.275-.121.537-.257.778-.407V8c0 .133-.058.318-.282.55-.227.237-.592.484-1.1.708C11.101 9.705 9.646 10 8 10c-1.647 0-3.102-.295-4.117-.742-.51-.224-.874-.47-1.101-.707C2.558 8.318 2.5 8.133 2.5 8zm0 2.225V12.5c0 .133.058.318.282.55.227.237.592.484 1.1.708 1.016.447 2.471.742 4.118.742 1.647 0 3.102-.295 4.117-.742.51-.224.874-.47 1.101-.707.224-.233.282-.418.282-.551v-2.275c-.241.15-.503.285-.778.406-1.247.549-2.917.869-4.722.869-1.805 0-3.475-.32-4.721-.869a6.236 6.236 0 01-.779-.406z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitBranch;
impl IconShape for GoGitBranch {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122V6A2.5 2.5 0 0110 8.5H6a1 1 0 00-1 1v1.128a2.251 2.251 0 11-1.5 0V5.372a2.25 2.25 0 111.5 0v1.836A2.492 2.492 0 016 7h4a1 1 0 001-1v-.628A2.25 2.25 0 019.5 3.25zM4.25 12a.75.75 0 100 1.5.75.75 0 000-1.5zM3.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBellSlash;
impl IconShape for GoBellSlash {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1.5c-.997 0-1.895.416-2.534 1.086A.75.75 0 014.38 1.55 5 5 0 0113 5v2.373a.75.75 0 01-1.5 0V5A3.5 3.5 0 008 1.5zM4.182 4.31L1.19 2.143a.75.75 0 10-.88 1.214L3 5.305v2.642a.25.25 0 01-.042.139L1.255 10.64A1.518 1.518 0 002.518 13h11.108l1.184.857a.75.75 0 10.88-1.214l-1.375-.996a1.196 1.196 0 00-.013-.01L4.198 4.321a.733.733 0 00-.016-.011zm7.373 7.19L4.5 6.391v1.556c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01.015.015 0 00.005.012.017.017 0 00.006.004l.007.001h9.037zM8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitCompare;
impl IconShape for GoGitCompare {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.573.677L7.177 3.073a.25.25 0 000 .354l2.396 2.396A.25.25 0 0010 5.646V4h1a1 1 0 011 1v5.628a2.251 2.251 0 101.5 0V5A2.5 2.5 0 0011 2.5h-1V.854a.25.25 0 00-.427-.177zM6 12v-1.646a.25.25 0 01.427-.177l2.396 2.396a.25.25 0 010 .354l-2.396 2.396A.25.25 0 016 15.146V13.5H5A2.5 2.5 0 012.5 11V5.372a2.25 2.25 0 111.5 0V11a1 1 0 001 1h1zm6.75 0a.75.75 0 100 1.5.75.75 0 000-1.5zM4 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedStar;
impl IconShape for GoFeedStar {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm.252-12.932a.478.478 0 00-.682.195l-1.2 2.432-2.684.39a.478.478 0 00-.266.816l1.944 1.892-.46 2.674a.478.478 0 00.694.504L8 10.709l2.4 1.261a.478.478 0 00.694-.504l-.458-2.673L12.578 6.9a.479.479 0 00-.265-.815l-2.685-.39-1.2-2.432a.478.478 0 00-.176-.195z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHubot;
impl IconShape for GoHubot {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 8a8 8 0 1116 0v5.25a.75.75 0 01-1.5 0V8a6.5 6.5 0 10-13 0v5.25a.75.75 0 01-1.5 0V8zm5.5 4.25a.75.75 0 01.75-.75h3.5a.75.75 0 010 1.5h-3.5a.75.75 0 01-.75-.75zM3 6.75C3 5.784 3.784 5 4.75 5h6.5c.966 0 1.75.784 1.75 1.75v1.5A1.75 1.75 0 0111.25 10h-6.5A1.75 1.75 0 013 8.25v-1.5zm1.47-.53a.75.75 0 011.06 0l.97.97.97-.97a.75.75 0 011.06 0l.97.97.97-.97a.75.75 0 111.06 1.06l-1.5 1.5a.75.75 0 01-1.06 0L8 7.81l-.97.97a.75.75 0 01-1.06 0l-1.5-1.5a.75.75 0 010-1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepo;
impl IconShape for GoRepo {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLocation;
impl IconShape for GoLocation {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.536 3.464a5 5 0 010 7.072L8 14.07l-3.536-3.535a5 5 0 117.072-7.072v.001zm1.06 8.132a6.5 6.5 0 10-9.192 0l3.535 3.536a1.5 1.5 0 002.122 0l3.535-3.536zM8 9a2 2 0 100-4 2 2 0 000 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStarFill;
impl IconShape for GoStarFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedForked;
impl IconShape for GoFeedForked {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zM6 6.928a1.75 1.75 0 10-1 0V7.5A1.5 1.5 0 006.5 9h1v1.072a1.75 1.75 0 101 0V9h1A1.5 1.5 0 0011 7.5v-.572a1.75 1.75 0 10-1 0V7.5a.5.5 0 01-.5.5h-3a.5.5 0 01-.5-.5v-.572z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoXCircleFill;
impl IconShape for GoXCircleFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.343 13.657A8 8 0 1113.657 2.343 8 8 0 012.343 13.657zM6.03 4.97a.75.75 0 00-1.06 1.06L6.94 8 4.97 9.97a.75.75 0 101.06 1.06L8 9.06l1.97 1.97a.75.75 0 101.06-1.06L9.06 8l1.97-1.97a.75.75 0 10-1.06-1.06L8 6.94 6.03 4.97z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoColumns;
impl IconShape for GoColumns {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.75 0A1.75 1.75 0 001 1.75v12.5c0 .966.784 1.75 1.75 1.75h2.5A1.75 1.75 0 007 14.25V1.75A1.75 1.75 0 005.25 0h-2.5zM2.5 1.75a.25.25 0 01.25-.25h2.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25h-2.5a.25.25 0 01-.25-.25V1.75zM10.75 0A1.75 1.75 0 009 1.75v12.5c0 .966.784 1.75 1.75 1.75h2.5A1.75 1.75 0 0015 14.25V1.75A1.75 1.75 0 0013.25 0h-2.5zm-.25 1.75a.25.25 0 01.25-.25h2.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25h-2.5a.25.25 0 01-.25-.25V1.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMute;
impl IconShape for GoMute {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 2.75a.75.75 0 00-1.238-.57L3.472 5H1.75A1.75 1.75 0 000 6.75v2.5C0 10.216.784 11 1.75 11h1.723l3.289 2.82A.75.75 0 008 13.25V2.75zM4.238 6.32L6.5 4.38v7.24L4.238 9.68a.75.75 0 00-.488-.18h-2a.25.25 0 01-.25-.25v-2.5a.25.25 0 01.25-.25h2a.75.75 0 00.488-.18zm7.042-1.1a.75.75 0 10-1.06 1.06L11.94 8l-1.72 1.72a.75.75 0 101.06 1.06L13 9.06l1.72 1.72a.75.75 0 101.06-1.06L14.06 8l1.72-1.72a.75.75 0 00-1.06-1.06L13 6.94l-1.72-1.72z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowBoth;
impl IconShape for GoArrowBoth {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.72 3.72a.75.75 0 011.06 1.06L2.56 7h10.88l-2.22-2.22a.75.75 0 011.06-1.06l3.5 3.5a.75.75 0 010 1.06l-3.5 3.5a.75.75 0 11-1.06-1.06l2.22-2.22H2.56l2.22 2.22a.75.75 0 11-1.06 1.06l-3.5-3.5a.75.75 0 010-1.06l3.5-3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileBinary;
impl IconShape for GoFileBinary {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 1.75C4 .784 4.784 0 5.75 0h5.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v8.586A1.75 1.75 0 0114.25 15h-9a.75.75 0 010-1.5h9a.25.25 0 00.25-.25V6h-2.75A1.75 1.75 0 0110 4.25V1.5H5.75a.25.25 0 00-.25.25v2a.75.75 0 01-1.5 0v-2zm7.5-.188V4.25c0 .138.112.25.25.25h2.688a.252.252 0 00-.011-.013l-2.914-2.914a.272.272 0 00-.013-.011zM0 7.75C0 6.784.784 6 1.75 6h1.5C4.216 6 5 6.784 5 7.75v2.5A1.75 1.75 0 013.25 12h-1.5A1.75 1.75 0 010 10.25v-2.5zm1.75-.25a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h1.5a.25.25 0 00.25-.25v-2.5a.25.25 0 00-.25-.25h-1.5zm5-1.5a.75.75 0 000 1.5h.75v3h-.75a.75.75 0 000 1.5h3a.75.75 0 000-1.5H9V6.75A.75.75 0 008.25 6h-1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiff;
impl IconShape for GoDiff {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.75 1.75a.75.75 0 00-1.5 0V5H4a.75.75 0 000 1.5h3.25v3.25a.75.75 0 001.5 0V6.5H12A.75.75 0 0012 5H8.75V1.75zM4 13a.75.75 0 000 1.5h8a.75.75 0 100-1.5H4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCopilot;
impl IconShape for GoCopilot {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.25 9a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5A.75.75 0 016.25 9zm4.25.75a.75.75 0 00-1.5 0v1.5a.75.75 0 001.5 0v-1.5z",
            }
            path {
                d: "M7.86 1.77c.05.053.097.107.14.164.043-.057.09-.111.14-.164.681-.731 1.737-.9 2.943-.765 1.23.136 2.145.527 2.724 1.26.566.716.693 1.614.693 2.485 0 .572-.053 1.147-.254 1.655l.168.838.066.033A2.75 2.75 0 0116 9.736V11c0 .24-.086.438-.156.567a2.173 2.173 0 01-.259.366c-.18.21-.404.413-.605.58a10.373 10.373 0 01-.792.597l-.015.01-.006.004-.028.018a8.832 8.832 0 01-.456.281c-.307.177-.749.41-1.296.642C11.296 14.528 9.756 15 8 15c-1.756 0-3.296-.472-4.387-.935a12.06 12.06 0 01-1.296-.641 8.815 8.815 0 01-.456-.281l-.028-.02-.006-.003-.015-.01a7.077 7.077 0 01-.235-.166c-.15-.108-.352-.26-.557-.43a5.19 5.19 0 01-.605-.58 2.167 2.167 0 01-.259-.367A1.19 1.19 0 010 11V9.736a2.75 2.75 0 011.52-2.46l.067-.033.167-.838C1.553 5.897 1.5 5.322 1.5 4.75c0-.87.127-1.77.693-2.485.579-.733 1.494-1.124 2.724-1.26 1.206-.134 2.262.034 2.944.765zM3.024 7.709L3 7.824v4.261c.02.013.043.025.065.038.264.152.65.356 1.134.562.972.412 2.307.815 3.801.815 1.494 0 2.83-.403 3.8-.815a10.6 10.6 0 001.2-.6v-4.26l-.023-.116c-.49.21-1.075.291-1.727.291-1.146 0-2.06-.328-2.71-.991A3.223 3.223 0 018 6.266c-.144.269-.321.52-.54.743C6.81 7.672 5.896 8 4.75 8c-.652 0-1.237-.082-1.727-.291zm3.741-4.916c-.193-.207-.637-.414-1.681-.298-1.02.114-1.48.404-1.713.7-.247.313-.37.79-.37 1.555 0 .792.129 1.17.308 1.37.162.181.52.38 1.442.38.854 0 1.339-.236 1.638-.54.315-.323.527-.827.618-1.553.117-.936-.038-1.396-.242-1.614zm2.472 0c.193-.207.637-.414 1.681-.298 1.02.114 1.48.404 1.713.7.247.313.37.79.37 1.555 0 .792-.129 1.17-.308 1.37-.162.181-.52.38-1.442.38-.854 0-1.339-.236-1.638-.54-.315-.323-.527-.827-.618-1.553-.117-.936.038-1.396.242-1.614z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoContainer;
impl IconShape for GoContainer {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.41.24l4.711 2.774A1.767 1.767 0 0116 4.54v5.01a1.77 1.77 0 01-.88 1.53l-7.753 4.521-.002.001a1.767 1.767 0 01-1.774 0H5.59L.873 12.85A1.762 1.762 0 010 11.327V6.292c0-.304.078-.598.22-.855l.004-.005.01-.019c.15-.262.369-.486.64-.643L8.641.239a1.75 1.75 0 011.765 0l.002.001zM9.397 1.534a.25.25 0 01.252 0l4.115 2.422-7.152 4.148a.267.267 0 01-.269 0L2.227 5.716l7.17-4.182zM7.365 9.402L8.73 8.61v4.46l-1.5.875V9.473a1.77 1.77 0 00.136-.071zm2.864 2.794V7.741l1.521-.882v4.45l-1.521.887zm3.021-1.762l1.115-.65h.002a.268.268 0 00.133-.232V5.264l-1.25.725v4.445zm-11.621 1.12l4.1 2.393V9.474a1.77 1.77 0 01-.138-.072L1.5 7.029v4.298c0 .095.05.181.129.227z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPaste;
impl IconShape for GoPaste {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.75 1a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-3a.75.75 0 00-.75-.75h-4.5zm.75 3V2.5h3V4h-3zm-2.874-.467a.75.75 0 00-.752-1.298A1.75 1.75 0 002 3.75v9.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-9.5a1.75 1.75 0 00-.874-1.515.75.75 0 10-.752 1.298.25.25 0 01.126.217v9.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-9.5a.25.25 0 01.126-.217z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHistory;
impl IconShape for GoHistory {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.643 3.143L.427 1.927A.25.25 0 000 2.104V5.75c0 .138.112.25.25.25h3.646a.25.25 0 00.177-.427L2.715 4.215a6.5 6.5 0 11-1.18 4.458.75.75 0 10-1.493.154 8.001 8.001 0 101.6-5.684zM7.75 4a.75.75 0 01.75.75v2.992l2.028.812a.75.75 0 01-.557 1.392l-2.5-1A.75.75 0 017 8.25v-3.5A.75.75 0 017.75 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPerson;
impl IconShape for GoPerson {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.5 5a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0zm.061 3.073a4 4 0 10-5.123 0 6.004 6.004 0 00-3.431 5.142.75.75 0 001.498.07 4.5 4.5 0 018.99 0 .75.75 0 101.498-.07 6.005 6.005 0 00-3.432-5.142z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLogoGithub;
impl IconShape for GoLogoGithub {
    fn view_box(&self) -> String {
        String::from("0 0 45 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.53 12.03h-.02c.009 0 .015.01.024.011h.006l-.01-.01zm.004.011c-.093.001-.327.05-.574.05-.78 0-1.05-.36-1.05-.83V8.13h1.59c.09 0 .16-.08.16-.19v-1.7c0-.09-.08-.17-.16-.17h-1.59V3.96c0-.08-.05-.13-.14-.13h-2.16c-.09 0-.14.05-.14.13v2.17s-1.09.27-1.16.28c-.08.02-.13.09-.13.17v1.36c0 .11.08.19.17.19h1.11v3.28c0 2.44 1.7 2.69 2.86 2.69.53 0 1.17-.17 1.27-.22.06-.02.09-.09.09-.16v-1.5a.177.177 0 00-.146-.18zM42.23 9.84c0-1.81-.73-2.05-1.5-1.97-.6.04-1.08.34-1.08.34v3.52s.49.34 1.22.36c1.03.03 1.36-.34 1.36-2.25zm2.43-.16c0 3.43-1.11 4.41-3.05 4.41-1.64 0-2.52-.83-2.52-.83s-.04.46-.09.52c-.03.06-.08.08-.14.08h-1.48c-.1 0-.19-.08-.19-.17l.02-11.11c0-.09.08-.17.17-.17h2.13c.09 0 .17.08.17.17v3.77s.82-.53 2.02-.53l-.01-.02c1.2 0 2.97.45 2.97 3.88zm-8.72-3.61h-2.1c-.11 0-.17.08-.17.19v5.44s-.55.39-1.3.39-.97-.34-.97-1.09V6.25c0-.09-.08-.17-.17-.17h-2.14c-.09 0-.17.08-.17.17v5.11c0 2.2 1.23 2.75 2.92 2.75 1.39 0 2.52-.77 2.52-.77s.05.39.08.45c.02.05.09.09.16.09h1.34c.11 0 .17-.08.17-.17l.02-7.47c0-.09-.08-.17-.19-.17zm-23.7-.01h-2.13c-.09 0-.17.09-.17.2v7.34c0 .2.13.27.3.27h1.92c.2 0 .25-.09.25-.27V6.23c0-.09-.08-.17-.17-.17zm-1.05-3.38c-.77 0-1.38.61-1.38 1.38 0 .77.61 1.38 1.38 1.38.75 0 1.36-.61 1.36-1.38 0-.77-.61-1.38-1.36-1.38zm16.49-.25h-2.11c-.09 0-.17.08-.17.17v4.09h-3.31V2.6c0-.09-.08-.17-.17-.17h-2.13c-.09 0-.17.08-.17.17v11.11c0 .09.09.17.17.17h2.13c.09 0 .17-.08.17-.17V8.96h3.31l-.02 4.75c0 .09.08.17.17.17h2.13c.09 0 .17-.08.17-.17V2.6c0-.09-.08-.17-.17-.17zM8.81 7.35v5.74c0 .04-.01.11-.06.13 0 0-1.25.89-3.31.89-2.49 0-5.44-.78-5.44-5.92S2.58 1.99 5.1 2c2.18 0 3.06.49 3.2.58.04.05.06.09.06.14L7.94 4.5c0 .09-.09.2-.2.17-.36-.11-.9-.33-2.17-.33-1.47 0-3.05.42-3.05 3.73s1.5 3.7 2.58 3.7c.92 0 1.25-.11 1.25-.11v-2.3H4.88c-.11 0-.19-.08-.19-.17V7.35c0-.09.08-.17.19-.17h3.74c.11 0 .19.08.19.17z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBookmark;
impl IconShape for GoBookmark {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.75 2.5a.25.25 0 00-.25.25v9.91l3.023-2.489a.75.75 0 01.954 0l3.023 2.49V2.75a.25.25 0 00-.25-.25h-6.5zM3 2.75C3 1.784 3.784 1 4.75 1h6.5c.966 0 1.75.784 1.75 1.75v11.5a.75.75 0 01-1.227.579L8 11.722l-3.773 3.107A.75.75 0 013 14.25V2.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiffModified;
impl IconShape for GoDiffModified {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.75 2.5h10.5a.25.25 0 01.25.25v10.5a.25.25 0 01-.25.25H2.75a.25.25 0 01-.25-.25V2.75a.25.25 0 01.25-.25zM13.25 1H2.75A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1zM8 10a2 2 0 100-4 2 2 0 000 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBug;
impl IconShape for GoBug {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.72.22a.75.75 0 011.06 0l1 .999a3.492 3.492 0 012.441 0l.999-1a.75.75 0 111.06 1.061l-.775.776c.616.63.995 1.493.995 2.444v.327c0 .1-.009.197-.025.292.408.14.764.392 1.029.722l1.968-.787a.75.75 0 01.556 1.392L13 7.258V9h2.25a.75.75 0 010 1.5H13v.5c0 .409-.049.806-.141 1.186l2.17.868a.75.75 0 01-.557 1.392l-2.184-.873A4.997 4.997 0 018 16a4.997 4.997 0 01-4.288-2.427l-2.183.873a.75.75 0 01-.558-1.392l2.17-.868A5.013 5.013 0 013 11v-.5H.75a.75.75 0 010-1.5H3V7.258L.971 6.446a.75.75 0 01.558-1.392l1.967.787c.265-.33.62-.583 1.03-.722a1.684 1.684 0 01-.026-.292V4.5c0-.951.38-1.814.995-2.444L4.72 1.28a.75.75 0 010-1.06zM6.173 5h3.654A.173.173 0 0010 4.827V4.5a2 2 0 10-4 0v.327c0 .096.077.173.173.173zM5.25 6.5a.75.75 0 00-.75.75V11a3.5 3.5 0 107 0V7.25a.75.75 0 00-.75-.75h-5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedPerson;
impl IconShape for GoFeedPerson {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm.847-8.145a2.502 2.502 0 10-1.694 0C5.471 8.261 4 9.775 4 11c0 .395.145.995 1 .995h6c.855 0 1-.6 1-.995 0-1.224-1.47-2.74-3.153-3.145z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLightBulb;
impl IconShape for GoLightBulb {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1.5c-2.363 0-4 1.69-4 3.75 0 .984.424 1.625.984 2.304l.214.253c.223.264.47.556.673.848.284.411.537.896.621 1.49a.75.75 0 01-1.484.211c-.04-.282-.163-.547-.37-.847a8.695 8.695 0 00-.542-.68c-.084-.1-.173-.205-.268-.32C3.201 7.75 2.5 6.766 2.5 5.25 2.5 2.31 4.863 0 8 0s5.5 2.31 5.5 5.25c0 1.516-.701 2.5-1.328 3.259-.095.115-.184.22-.268.319-.207.245-.383.453-.541.681-.208.3-.33.565-.37.847a.75.75 0 01-1.485-.212c.084-.593.337-1.078.621-1.489.203-.292.45-.584.673-.848.075-.088.147-.173.213-.253.561-.679.985-1.32.985-2.304 0-2.06-1.637-3.75-4-3.75zM6 15.25a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5a.75.75 0 01-.75-.75zM5.75 12a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowLeft;
impl IconShape for GoArrowLeft {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.78 12.53a.75.75 0 01-1.06 0L2.47 8.28a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 1.06L4.81 7h7.44a.75.75 0 010 1.5H4.81l2.97 2.97a.75.75 0 010 1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoItalic;
impl IconShape for GoItalic {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 2.75A.75.75 0 016.75 2h6.5a.75.75 0 010 1.5h-2.505l-3.858 9H9.25a.75.75 0 010 1.5h-6.5a.75.75 0 010-1.5h2.505l3.858-9H6.75A.75.75 0 016 2.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBold;
impl IconShape for GoBold {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 2a1 1 0 00-1 1v10a1 1 0 001 1h5.5a3.5 3.5 0 001.852-6.47A3.5 3.5 0 008.5 2H4zm4.5 5a1.5 1.5 0 100-3H5v3h3.5zM5 9v3h4.5a1.5 1.5 0 000-3H5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSquareFill;
impl IconShape for GoSquareFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.75 4A1.75 1.75 0 004 5.75v4.5c0 .966.784 1.75 1.75 1.75h4.5A1.75 1.75 0 0012 10.25v-4.5A1.75 1.75 0 0010.25 4h-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoUnfold;
impl IconShape for GoUnfold {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.177.677l2.896 2.896a.25.25 0 01-.177.427H8.75v1.25a.75.75 0 01-1.5 0V4H5.104a.25.25 0 01-.177-.427L7.823.677a.25.25 0 01.354 0zM7.25 10.75a.75.75 0 011.5 0V12h2.146a.25.25 0 01.177.427l-2.896 2.896a.25.25 0 01-.354 0l-2.896-2.896A.25.25 0 015.104 12H7.25v-1.25zm-5-2a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM6 8a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5A.75.75 0 016 8zm2.25.75a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM12 8a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5A.75.75 0 0112 8zm2.25.75a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSun;
impl IconShape for GoSun {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 10.5a2.5 2.5 0 100-5 2.5 2.5 0 000 5zM8 12a4 4 0 100-8 4 4 0 000 8zM8 0a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0V.75A.75.75 0 018 0zm0 13a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5A.75.75 0 018 13zM2.343 2.343a.75.75 0 011.061 0l1.06 1.061a.75.75 0 01-1.06 1.06l-1.06-1.06a.75.75 0 010-1.06zm9.193 9.193a.75.75 0 011.06 0l1.061 1.06a.75.75 0 01-1.06 1.061l-1.061-1.06a.75.75 0 010-1.061zM16 8a.75.75 0 01-.75.75h-1.5a.75.75 0 010-1.5h1.5A.75.75 0 0116 8zM3 8a.75.75 0 01-.75.75H.75a.75.75 0 010-1.5h1.5A.75.75 0 013 8zm10.657-5.657a.75.75 0 010 1.061l-1.061 1.06a.75.75 0 11-1.06-1.06l1.06-1.06a.75.75 0 011.06 0zm-9.193 9.193a.75.75 0 010 1.06l-1.06 1.061a.75.75 0 11-1.061-1.06l1.06-1.061a.75.75 0 011.061 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoPush;
impl IconShape for GoRepoPush {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 2.5A2.5 2.5 0 013.5 0h8.75a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0V1.5h-8a1 1 0 00-1 1v6.708A2.492 2.492 0 013.5 9h3.25a.75.75 0 010 1.5H3.5a1 1 0 100 2h5.75a.75.75 0 010 1.5H3.5A2.5 2.5 0 011 11.5v-9zm13.23 7.79a.75.75 0 001.06-1.06l-2.505-2.505a.75.75 0 00-1.06 0L9.22 9.229a.75.75 0 001.06 1.061l1.225-1.224v6.184a.75.75 0 001.5 0V9.066l1.224 1.224z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoKey;
impl IconShape for GoKey {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.5 5.5a4 4 0 112.731 3.795.75.75 0 00-.768.18L7.44 10.5H6.25a.75.75 0 00-.75.75v1.19l-.06.06H4.25a.75.75 0 00-.75.75v1.19l-.06.06H1.75a.25.25 0 01-.25-.25v-1.69l5.024-5.023a.75.75 0 00.181-.768A3.995 3.995 0 016.5 5.5zm4-5.5a5.5 5.5 0 00-5.348 6.788L.22 11.72a.75.75 0 00-.22.53v2C0 15.216.784 16 1.75 16h2a.75.75 0 00.53-.22l.5-.5a.75.75 0 00.22-.53V14h.75a.75.75 0 00.53-.22l.5-.5a.75.75 0 00.22-.53V12h.75a.75.75 0 00.53-.22l.932-.932A5.5 5.5 0 1010.5 0zm.5 6a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoChevronLeft;
impl IconShape for GoChevronLeft {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.78 12.78a.75.75 0 01-1.06 0L4.47 8.53a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 1.06L6.06 8l3.72 3.72a.75.75 0 010 1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileDirectory;
impl IconShape for GoFileDirectory {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-8.5a.25.25 0 00-.25-.25H7.5c-.55 0-1.07-.26-1.4-.7l-.9-1.2a.25.25 0 00-.2-.1H1.75zM0 2.75C0 1.784.784 1 1.75 1H5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 00.2.1h6.75c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25V2.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileDirectoryOpenFill;
impl IconShape for GoFileDirectoryOpenFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M.513 1.513A1.75 1.75 0 011.75 1h3.5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 00.2.1H13a1 1 0 011 1v.5H2.75a.75.75 0 000 1.5h11.978a1 1 0 01.994 1.117L15 13.25A1.75 1.75 0 0113.25 15H1.75A1.75 1.75 0 010 13.25V2.75c0-.464.184-.91.513-1.237z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowDown;
impl IconShape for GoArrowDown {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.03 8.22a.75.75 0 010 1.06l-4.25 4.25a.75.75 0 01-1.06 0L3.47 9.28a.75.75 0 011.06-1.06l2.97 2.97V3.75a.75.75 0 011.5 0v7.44l2.97-2.97a.75.75 0 011.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoNote;
impl IconShape for GoNote {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 3.75C0 2.784.784 2 1.75 2h12.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 14H1.75A1.75 1.75 0 010 12.25v-8.5zm1.75-.25a.25.25 0 00-.25.25v8.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-8.5a.25.25 0 00-.25-.25H1.75zM3.5 6.25a.75.75 0 01.75-.75h7a.75.75 0 010 1.5h-7a.75.75 0 01-.75-.75zm.75 2.25a.75.75 0 000 1.5h4a.75.75 0 000-1.5h-4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoChecklist;
impl IconShape for GoChecklist {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.5 1.75a.25.25 0 01.25-.25h8.5a.25.25 0 01.25.25v7.736a.75.75 0 101.5 0V1.75A1.75 1.75 0 0011.25 0h-8.5A1.75 1.75 0 001 1.75v11.5c0 .966.784 1.75 1.75 1.75h3.17a.75.75 0 000-1.5H2.75a.25.25 0 01-.25-.25V1.75zM4.75 4a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5zM4 7.75A.75.75 0 014.75 7h2a.75.75 0 010 1.5h-2A.75.75 0 014 7.75zm11.774 3.537a.75.75 0 00-1.048-1.074L10.7 14.145 9.281 12.72a.75.75 0 00-1.062 1.058l1.943 1.95a.75.75 0 001.055.008l4.557-4.45z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGear;
impl IconShape for GoGear {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.429 1.525a6.593 6.593 0 011.142 0c.036.003.108.036.137.146l.289 1.105c.147.56.55.967.997 1.189.174.086.341.183.501.29.417.278.97.423 1.53.27l1.102-.303c.11-.03.175.016.195.046.219.31.41.641.573.989.014.031.022.11-.059.19l-.815.806c-.411.406-.562.957-.53 1.456a4.588 4.588 0 010 .582c-.032.499.119 1.05.53 1.456l.815.806c.08.08.073.159.059.19a6.494 6.494 0 01-.573.99c-.02.029-.086.074-.195.045l-1.103-.303c-.559-.153-1.112-.008-1.529.27-.16.107-.327.204-.5.29-.449.222-.851.628-.998 1.189l-.289 1.105c-.029.11-.101.143-.137.146a6.613 6.613 0 01-1.142 0c-.036-.003-.108-.037-.137-.146l-.289-1.105c-.147-.56-.55-.967-.997-1.189a4.502 4.502 0 01-.501-.29c-.417-.278-.97-.423-1.53-.27l-1.102.303c-.11.03-.175-.016-.195-.046a6.492 6.492 0 01-.573-.989c-.014-.031-.022-.11.059-.19l.815-.806c.411-.406.562-.957.53-1.456a4.587 4.587 0 010-.582c.032-.499-.119-1.05-.53-1.456l-.815-.806c-.08-.08-.073-.159-.059-.19a6.44 6.44 0 01.573-.99c.02-.029.086-.075.195-.045l1.103.303c.559.153 1.112.008 1.529-.27.16-.107.327-.204.5-.29.449-.222.851-.628.998-1.189l.289-1.105c.029-.11.101-.143.137-.146zM8 0c-.236 0-.47.01-.701.03-.743.065-1.29.615-1.458 1.261l-.29 1.106c-.017.066-.078.158-.211.224a5.994 5.994 0 00-.668.386c-.123.082-.233.09-.3.071L3.27 2.776c-.644-.177-1.392.02-1.82.63a7.977 7.977 0 00-.704 1.217c-.315.675-.111 1.422.363 1.891l.815.806c.05.048.098.147.088.294a6.084 6.084 0 000 .772c.01.147-.038.246-.088.294l-.815.806c-.474.469-.678 1.216-.363 1.891.2.428.436.835.704 1.218.428.609 1.176.806 1.82.63l1.103-.303c.066-.019.176-.011.299.071.213.143.436.272.668.386.133.066.194.158.212.224l.289 1.106c.169.646.715 1.196 1.458 1.26a8.094 8.094 0 001.402 0c.743-.064 1.29-.614 1.458-1.26l.29-1.106c.017-.066.078-.158.211-.224a5.98 5.98 0 00.668-.386c.123-.082.233-.09.3-.071l1.102.302c.644.177 1.392-.02 1.82-.63.268-.382.505-.789.704-1.217.315-.675.111-1.422-.364-1.891l-.814-.806c-.05-.048-.098-.147-.088-.294a6.1 6.1 0 000-.772c-.01-.147.039-.246.088-.294l.814-.806c.475-.469.679-1.216.364-1.891a7.992 7.992 0 00-.704-1.218c-.428-.609-1.176-.806-1.82-.63l-1.103.303c-.066.019-.176.011-.299-.071a5.991 5.991 0 00-.668-.386c-.133-.066-.194-.158-.212-.224L10.16 1.29C9.99.645 9.444.095 8.701.031A8.094 8.094 0 008 0zm1.5 8a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM11 8a3 3 0 11-6 0 3 3 0 016 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDeviceDesktop;
impl IconShape for GoDeviceDesktop {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 2.5h12.5a.25.25 0 01.25.25v7.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25v-7.5a.25.25 0 01.25-.25zM14.25 1H1.75A1.75 1.75 0 000 2.75v7.5C0 11.216.784 12 1.75 12h3.727c-.1 1.041-.52 1.872-1.292 2.757A.75.75 0 004.75 16h6.5a.75.75 0 00.565-1.243c-.772-.885-1.193-1.716-1.292-2.757h3.727A1.75 1.75 0 0016 10.25v-7.5A1.75 1.75 0 0014.25 1zM9.018 12H6.982a5.72 5.72 0 01-.765 2.5h3.566a5.72 5.72 0 01-.765-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShield;
impl IconShape for GoShield {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.467.133a1.75 1.75 0 011.066 0l5.25 1.68A1.75 1.75 0 0115 3.48V7c0 1.566-.32 3.182-1.303 4.682-.983 1.498-2.585 2.813-5.032 3.855a1.7 1.7 0 01-1.33 0c-2.447-1.042-4.049-2.357-5.032-3.855C1.32 10.182 1 8.566 1 7V3.48a1.75 1.75 0 011.217-1.667l5.25-1.68zm.61 1.429a.25.25 0 00-.153 0l-5.25 1.68a.25.25 0 00-.174.238V7c0 1.358.275 2.666 1.057 3.86.784 1.194 2.121 2.34 4.366 3.297a.2.2 0 00.154 0c2.245-.956 3.582-2.104 4.366-3.298C13.225 9.666 13.5 8.36 13.5 7V3.48a.25.25 0 00-.174-.237l-5.25-1.68zM9 10.5a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.75a.75.75 0 10-1.5 0v3a.75.75 0 001.5 0v-3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodeReview;
impl IconShape for GoCodeReview {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 2.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v8.5a.25.25 0 01-.25.25h-6.5a.75.75 0 00-.53.22L4.5 14.44v-2.19a.75.75 0 00-.75-.75h-2a.25.25 0 01-.25-.25v-8.5zM1.75 1A1.75 1.75 0 000 2.75v8.5C0 12.216.784 13 1.75 13H3v1.543a1.457 1.457 0 002.487 1.03L8.061 13h6.189A1.75 1.75 0 0016 11.25v-8.5A1.75 1.75 0 0014.25 1H1.75zm5.03 3.47a.75.75 0 010 1.06L5.31 7l1.47 1.47a.75.75 0 01-1.06 1.06l-2-2a.75.75 0 010-1.06l2-2a.75.75 0 011.06 0zm2.44 0a.75.75 0 000 1.06L10.69 7 9.22 8.47a.75.75 0 001.06 1.06l2-2a.75.75 0 000-1.06l-2-2a.75.75 0 00-1.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCloud;
impl IconShape for GoCloud {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 7.25A5.225 5.225 0 017.25 2a5.222 5.222 0 014.767 3.029A4.472 4.472 0 0116 9.5c0 2.505-1.995 4.5-4.5 4.5h-8A3.475 3.475 0 010 10.5c0-1.41.809-2.614 2.001-3.17L2 7.25zm1.54.482a.75.75 0 01-.556.832c-.86.22-1.484.987-1.484 1.936 0 1.124.876 2 2 2h8c1.676 0 3-1.324 3-3s-1.324-3-3-3a.75.75 0 01-.709-.504A3.72 3.72 0 007.25 3.5C5.16 3.5 3.5 5.16 3.5 7.25a3.276 3.276 0 00.035.436l.004.036.001.008v.002z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTriangleRight;
impl IconShape for GoTriangleRight {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.427 4.427l3.396 3.396a.25.25 0 010 .354l-3.396 3.396A.25.25 0 016 11.396V4.604a.25.25 0 01.427-.177z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoEyeClosed;
impl IconShape for GoEyeClosed {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M.143 2.31a.75.75 0 011.047-.167l14.5 10.5a.75.75 0 11-.88 1.214l-2.248-1.628C11.346 13.19 9.792 14 8 14c-1.981 0-3.67-.992-4.933-2.078C1.797 10.832.88 9.577.43 8.9a1.618 1.618 0 010-1.797c.353-.533.995-1.42 1.868-2.305L.31 3.357A.75.75 0 01.143 2.31zm3.386 3.378a14.21 14.21 0 00-1.85 2.244.12.12 0 00-.022.068c0 .021.006.045.022.068.412.621 1.242 1.75 2.366 2.717C5.175 11.758 6.527 12.5 8 12.5c1.195 0 2.31-.488 3.29-1.191L9.063 9.695A2 2 0 016.058 7.52l-2.53-1.832zM8 3.5c-.516 0-1.017.09-1.499.251a.75.75 0 11-.473-1.423A6.23 6.23 0 018 2c1.981 0 3.67.992 4.933 2.078 1.27 1.091 2.187 2.345 2.637 3.023a1.619 1.619 0 010 1.798c-.11.166-.248.365-.41.587a.75.75 0 11-1.21-.887c.148-.201.272-.382.371-.53a.119.119 0 000-.137c-.412-.621-1.242-1.75-2.366-2.717C10.825 4.242 9.473 3.5 8 3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileDiff;
impl IconShape for GoFileDiff {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.75 1.5a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25h10.5a.25.25 0 00.25-.25V4.664a.25.25 0 00-.073-.177l-2.914-2.914a.25.25 0 00-.177-.073H2.75zM1 1.75C1 .784 1.784 0 2.75 0h7.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0113.25 16H2.75A1.75 1.75 0 011 14.25V1.75zm7 1.5a.75.75 0 01.75.75v1.5h1.5a.75.75 0 010 1.5h-1.5v1.5a.75.75 0 01-1.5 0V7h-1.5a.75.75 0 010-1.5h1.5V4A.75.75 0 018 3.25zm-3 8a.75.75 0 01.75-.75h4.5a.75.75 0 010 1.5h-4.5a.75.75 0 01-.75-.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDot;
impl IconShape for GoDot {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 5.5a2.5 2.5 0 100 5 2.5 2.5 0 000-5zM4 8a4 4 0 118 0 4 4 0 01-8 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoThumbsup;
impl IconShape for GoThumbsup {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.834.066C7.494-.087 6.5 1.048 6.5 2.25v.5c0 1.329-.647 2.124-1.318 2.614-.328.24-.66.403-.918.508A1.75 1.75 0 002.75 5h-1A1.75 1.75 0 000 6.75v7.5C0 15.216.784 16 1.75 16h1a1.75 1.75 0 001.662-1.201c.525.075 1.067.229 1.725.415.152.043.31.088.475.133 1.154.32 2.54.653 4.388.653 1.706 0 2.97-.153 3.722-1.14.353-.463.537-1.042.668-1.672.118-.56.208-1.243.313-2.033l.04-.306c.25-1.869.265-3.318-.188-4.316a2.418 2.418 0 00-1.137-1.2C13.924 5.085 13.353 5 12.75 5h-1.422l.015-.113c.07-.518.157-1.17.157-1.637 0-.922-.151-1.719-.656-2.3-.51-.589-1.247-.797-2.01-.884zM4.5 13.3c.705.088 1.39.284 2.072.478l.441.125c1.096.305 2.334.598 3.987.598 1.794 0 2.28-.223 2.528-.549.147-.193.276-.505.394-1.07.105-.502.188-1.124.295-1.93l.04-.3c.25-1.882.189-2.933-.068-3.497a.922.922 0 00-.442-.48c-.208-.104-.52-.174-.997-.174H11c-.686 0-1.295-.577-1.206-1.336.023-.192.05-.39.076-.586.065-.488.13-.97.13-1.328 0-.809-.144-1.15-.288-1.316-.137-.158-.402-.304-1.048-.378C8.357 1.521 8 1.793 8 2.25v.5c0 1.922-.978 3.128-1.933 3.825a5.861 5.861 0 01-1.567.81V13.3zM2.75 6.5a.25.25 0 01.25.25v7.5a.25.25 0 01-.25.25h-1a.25.25 0 01-.25-.25v-7.5a.25.25 0 01.25-.25h1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRocket;
impl IconShape for GoRocket {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.064 0a8.75 8.75 0 00-6.187 2.563l-.459.458c-.314.314-.616.641-.904.979H3.31a1.75 1.75 0 00-1.49.833L.11 7.607a.75.75 0 00.418 1.11l3.102.954c.037.051.079.1.124.145l2.429 2.428c.046.046.094.088.145.125l.954 3.102a.75.75 0 001.11.418l2.774-1.707a1.75 1.75 0 00.833-1.49V9.485c.338-.288.665-.59.979-.904l.458-.459A8.75 8.75 0 0016 1.936V1.75A1.75 1.75 0 0014.25 0h-.186zM10.5 10.625c-.088.06-.177.118-.266.175l-2.35 1.521.548 1.783 1.949-1.2a.25.25 0 00.119-.213v-2.066zM3.678 8.116L5.2 5.766c.058-.09.117-.178.176-.266H3.309a.25.25 0 00-.213.119l-1.2 1.95 1.782.547zm5.26-4.493A7.25 7.25 0 0114.063 1.5h.186a.25.25 0 01.25.25v.186a7.25 7.25 0 01-2.123 5.127l-.459.458a15.21 15.21 0 01-2.499 2.02l-2.317 1.5-2.143-2.143 1.5-2.317a15.25 15.25 0 012.02-2.5l.458-.458h.002zM12 5a1 1 0 11-2 0 1 1 0 012 0zm-8.44 9.56a1.5 1.5 0 10-2.12-2.12c-.734.73-1.047 2.332-1.15 3.003a.23.23 0 00.265.265c.671-.103 2.273-.416 3.005-1.148z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStop;
impl IconShape for GoStop {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.47.22A.75.75 0 015 0h6a.75.75 0 01.53.22l4.25 4.25c.141.14.22.331.22.53v6a.75.75 0 01-.22.53l-4.25 4.25A.75.75 0 0111 16H5a.75.75 0 01-.53-.22L.22 11.53A.75.75 0 010 11V5a.75.75 0 01.22-.53L4.47.22zm.84 1.28L1.5 5.31v5.38l3.81 3.81h5.38l3.81-3.81V5.31L10.69 1.5H5.31zM8 4a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 018 4zm0 8a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileSymlinkFile;
impl IconShape for GoFileSymlinkFile {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 1.75C2 .784 2.784 0 3.75 0h5.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v8.586A1.75 1.75 0 0112.25 15h-7a.75.75 0 010-1.5h7a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75a.25.25 0 00-.25.25V4.5a.75.75 0 01-1.5 0V1.75zm7.5-.188V4.25c0 .138.112.25.25.25h2.688a.252.252 0 00-.011-.013L9.513 1.573a.248.248 0 00-.013-.011zm-8 10.675a2.25 2.25 0 012.262-2.25L4 9.99v1.938c0 .218.26.331.42.183l2.883-2.677a.25.25 0 000-.366L4.42 6.39a.25.25 0 00-.42.183V8.49l-.23-.001A3.75 3.75 0 000 12.238v1.012a.75.75 0 001.5 0v-1.013z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoChevronDown;
impl IconShape for GoChevronDown {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12.78 6.22a.75.75 0 010 1.06l-4.25 4.25a.75.75 0 01-1.06 0L3.22 7.28a.75.75 0 011.06-1.06L8 9.94l3.72-3.72a.75.75 0 011.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoAccessibility;
impl IconShape for GoAccessibility {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.923 5.302a3 3 0 10-3.847 0A2.713 2.713 0 005.9 5.5H2A.75.75 0 002 7h3.3l-.578 5.163-.362 2.997a.75.75 0 101.49.18L6.132 13h3.736l.282 2.34a.75.75 0 101.49-.18l-.362-2.997L10.7 7H14a.75.75 0 000-1.5h-3.899a2.697 2.697 0 00-.178-.198zM9.5 3a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zm-.3 4.073l.495 4.427h-3.39l.496-4.427a1.207 1.207 0 012.398 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTools;
impl IconShape for GoTools {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.433 2.304A4.494 4.494 0 003.5 6c0 1.598.832 3.002 2.09 3.802.518.328.929.923.902 1.64v.008l-.164 3.337a.75.75 0 11-1.498-.073l.163-3.33c.002-.085-.05-.216-.207-.316A5.996 5.996 0 012 6a5.994 5.994 0 012.567-4.92 1.482 1.482 0 011.673-.04c.462.296.76.827.76 1.423v2.82c0 .082.041.16.11.206l.75.51a.25.25 0 00.28 0l.75-.51A.25.25 0 009 5.282V2.463c0-.596.298-1.127.76-1.423a1.482 1.482 0 011.673.04A5.994 5.994 0 0114 6a5.996 5.996 0 01-2.786 5.068c-.157.1-.209.23-.207.315l.163 3.33a.75.75 0 11-1.498.074l-.164-3.345c-.027-.717.384-1.312.902-1.64A4.496 4.496 0 0012.5 6a4.494 4.494 0 00-1.933-3.696c-.024.017-.067.067-.067.16v2.818a1.75 1.75 0 01-.767 1.448l-.75.51a1.75 1.75 0 01-1.966 0l-.75-.51A1.75 1.75 0 015.5 5.282V2.463c0-.092-.043-.142-.067-.159zm.01-.005z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLogoGist;
impl IconShape for GoLogoGist {
    fn view_box(&self) -> String {
        String::from("0 0 25 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.7 8.73h2.45v4.02c-.55.27-1.64.34-2.53.34-2.56 0-3.47-2.2-3.47-5.05 0-2.85.91-5.06 3.48-5.06 1.28 0 2.06.23 3.28.73V2.66C7.27 2.33 6.25 2 4.63 2 1.13 2 0 4.69 0 8.03c0 3.34 1.11 6.03 4.63 6.03 1.64 0 2.81-.27 3.59-.64V7.73H4.7v1zm6.39 3.72V6.06h-1.05v6.28c0 1.25.58 1.72 1.72 1.72v-.89c-.48 0-.67-.16-.67-.7v-.02zm.25-8.72c0-.44-.33-.78-.78-.78s-.77.34-.77.78.33.78.77.78.78-.34.78-.78zm4.34 5.69c-1.5-.13-1.78-.48-1.78-1.17 0-.77.33-1.34 1.88-1.34 1.05 0 1.66.16 2.27.36v-.94c-.69-.3-1.52-.39-2.25-.39-2.2 0-2.92 1.2-2.92 2.31 0 1.08.47 1.88 2.73 2.08 1.55.13 1.77.63 1.77 1.34 0 .73-.44 1.42-2.06 1.42-1.11 0-1.86-.19-2.33-.36v.94c.5.2 1.58.39 2.33.39 2.38 0 3.14-1.2 3.14-2.41 0-1.28-.53-2.03-2.75-2.23h-.03zm8.58-2.47v-.86h-2.42v-2.5l-1.08.31v2.11l-1.56.44v.48h1.56v5c0 1.53 1.19 2.13 2.5 2.13.19 0 .52-.02.69-.05v-.89c-.19.03-.41.03-.61.03-.97 0-1.5-.39-1.5-1.34V6.94h2.42v.02-.01z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoVideo;
impl IconShape for GoVideo {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 3.5a.25.25 0 00-.25.25v8.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-8.5a.25.25 0 00-.25-.25H1.75zM0 3.75C0 2.784.784 2 1.75 2h12.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 14H1.75A1.75 1.75 0 010 12.25v-8.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M6 10.559V5.442a.25.25 0 01.379-.215l4.264 2.559a.25.25 0 010 .428l-4.264 2.559A.25.25 0 016 10.559z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCloudOffline;
impl IconShape for GoCloudOffline {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.25 2c-.69 0-1.351.13-1.957.371a.75.75 0 10.554 1.394c.43-.17.903-.265 1.403-.265a3.72 3.72 0 013.541 2.496.75.75 0 00.709.504c1.676 0 3 1.324 3 3a3 3 0 01-.681 1.92.75.75 0 001.156.955A4.496 4.496 0 0016 9.5a4.472 4.472 0 00-3.983-4.471A5.222 5.222 0 007.25 2z",
            }
            path {
                d: "M.72 1.72a.75.75 0 011.06 0l2.311 2.31c.03.025.056.052.08.08l8.531 8.532a.785.785 0 01.035.034l2.043 2.044a.75.75 0 11-1.06 1.06l-1.8-1.799a4.64 4.64 0 01-.42.019h-8A3.475 3.475 0 010 10.5c0-1.41.809-2.614 2.001-3.17a5.218 5.218 0 01.646-2.622L.72 2.78a.75.75 0 010-1.06zM3.5 7.25c0-.505.096-.983.271-1.418L10.44 12.5H3.5c-1.124 0-2-.876-2-2 0-.95.624-1.716 1.484-1.936a.75.75 0 00.557-.833A4.1 4.1 0 013.5 7.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBellFill;
impl IconShape for GoBellFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16c.9 0 1.7-.6 1.9-1.5.1-.3-.1-.5-.4-.5h-3c-.3 0-.5.2-.4.5.2.9 1 1.5 1.9 1.5zM3 5c0-2.8 2.2-5 5-5s5 2.2 5 5v3l1.7 2.6c.2.2.3.5.3.8 0 .8-.7 1.5-1.5 1.5h-11c-.8.1-1.5-.6-1.5-1.4 0-.3.1-.6.3-.8L3 8.1V5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileSubmodule;
impl IconShape for GoFileSubmodule {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 2.75C0 1.784.784 1 1.75 1H5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 00.2.1h6.75c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25V2.75zm9.42 9.36l2.883-2.677a.25.25 0 000-.366L9.42 6.39a.25.25 0 00-.42.183V8.5H4.75a.75.75 0 100 1.5H9v1.927c0 .218.26.331.42.183z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoVerified;
impl IconShape for GoVerified {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.585.52a2.678 2.678 0 00-3.17 0l-.928.68a1.178 1.178 0 01-.518.215L3.83 1.59a2.678 2.678 0 00-2.24 2.24l-.175 1.14a1.178 1.178 0 01-.215.518l-.68.928a2.678 2.678 0 000 3.17l.68.928c.113.153.186.33.215.518l.175 1.138a2.678 2.678 0 002.24 2.24l1.138.175c.187.029.365.102.518.215l.928.68a2.678 2.678 0 003.17 0l.928-.68a1.17 1.17 0 01.518-.215l1.138-.175a2.678 2.678 0 002.241-2.241l.175-1.138c.029-.187.102-.365.215-.518l.68-.928a2.678 2.678 0 000-3.17l-.68-.928a1.179 1.179 0 01-.215-.518L14.41 3.83a2.678 2.678 0 00-2.24-2.24l-1.138-.175a1.179 1.179 0 01-.518-.215L9.585.52zM7.303 1.728c.415-.305.98-.305 1.394 0l.928.68c.348.256.752.423 1.18.489l1.136.174c.51.078.909.478.987.987l.174 1.137c.066.427.233.831.489 1.18l.68.927c.305.415.305.98 0 1.394l-.68.928a2.678 2.678 0 00-.489 1.18l-.174 1.136a1.178 1.178 0 01-.987.987l-1.137.174a2.678 2.678 0 00-1.18.489l-.927.68c-.415.305-.98.305-1.394 0l-.928-.68a2.678 2.678 0 00-1.18-.489l-1.136-.174a1.178 1.178 0 01-.987-.987l-.174-1.137a2.678 2.678 0 00-.489-1.18l-.68-.927a1.178 1.178 0 010-1.394l.68-.928c.256-.348.423-.752.489-1.18l.174-1.136c.078-.51.478-.909.987-.987l1.137-.174a2.678 2.678 0 001.18-.489l.927-.68zM11.28 6.78a.75.75 0 00-1.06-1.06L7 8.94 5.78 7.72a.75.75 0 00-1.06 1.06l1.75 1.75a.75.75 0 001.06 0l3.75-3.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBriefcase;
impl IconShape for GoBriefcase {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.75 0A1.75 1.75 0 005 1.75V3H1.75A1.75 1.75 0 000 4.75v8.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3H11V1.75A1.75 1.75 0 009.25 0h-2.5zM9.5 3V1.75a.25.25 0 00-.25-.25h-2.5a.25.25 0 00-.25.25V3h3zM5 4.5H1.75a.25.25 0 00-.25.25V6a2 2 0 002 2h9a2 2 0 002-2V4.75a.25.25 0 00-.25-.25H5zm-1.5 5a3.484 3.484 0 01-2-.627v4.377c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V8.873a3.484 3.484 0 01-2 .627h-9z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoUnlock;
impl IconShape for GoUnlock {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.5 4a2.5 2.5 0 014.607-1.346.75.75 0 101.264-.808A4 4 0 004 4v2h-.501A1.5 1.5 0 002 7.5v6A1.5 1.5 0 003.5 15h9a1.5 1.5 0 001.5-1.5v-6A1.5 1.5 0 0012.5 6h-7V4zm-.75 3.5H3.5v6h9v-6H4.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileBadge;
impl IconShape for GoFileBadge {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h3.5a.75.75 0 010 1.5h-3.5A1.75 1.75 0 011 13.25V1.75C1 .784 1.784 0 2.75 0h8a1.75 1.75 0 011.508.862.75.75 0 11-1.289.768.25.25 0 00-.219-.13h-8z",
            }
            path {
                d: "M8 7a4 4 0 116.49 3.13l.995 4.973a.75.75 0 01-.991.852l-2.409-.876a.25.25 0 00-.17 0l-2.409.876a.75.75 0 01-.991-.852l.994-4.973A3.993 3.993 0 018 7zm4-2.5a2.5 2.5 0 100 5 2.5 2.5 0 000-5zm0 6.5a4 4 0 001.104-.154l.649 3.243-1.155-.42c-.386-.14-.81-.14-1.196 0l-1.155.42.649-3.243A4 4 0 0012 11z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiamond;
impl IconShape for GoDiamond {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M.527 9.237a1.75 1.75 0 010-2.474L6.777.512a1.75 1.75 0 012.475 0l6.251 6.25a1.751 1.751 0 010 2.475l-6.25 6.251a1.751 1.751 0 01-2.475 0L.527 9.238v-.001zm1.06-1.414a.25.25 0 000 .354l6.251 6.25a.25.25 0 00.354 0l6.25-6.25a.25.25 0 000-.354l-6.25-6.25a.25.25 0 00-.354 0l-6.25 6.25h-.001z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBrowser;
impl IconShape for GoBrowser {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 2.75C0 1.784.784 1 1.75 1h12.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25V2.75zm1.75-.25a.25.25 0 00-.25.25V4.5h2v-2H1.75zM5 2.5v2h2v-2H5zm3.5 0v2h6V2.75a.25.25 0 00-.25-.25H8.5zm6 3.5h-13v7.25c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDeviceCamera;
impl IconShape for GoDeviceCamera {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 3H7c0-.55-.45-1-1-1H2c-.55 0-1 .45-1 1-.55 0-1 .45-1 1v9c0 .55.45 1 1 1h14c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1zM6 5H2V4h4v1zm4.5 7C8.56 12 7 10.44 7 8.5S8.56 5 10.5 5 14 6.56 14 8.5 12.44 12 10.5 12zM13 8.5c0 1.38-1.13 2.5-2.5 2.5S8 9.87 8 8.5 9.13 6 10.5 6 13 7.13 13 8.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSmiley;
impl IconShape for GoSmiley {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zM5 8a1 1 0 100-2 1 1 0 000 2zm7-1a1 1 0 11-2 0 1 1 0 012 0zM5.32 9.636a.75.75 0 011.038.175l.007.009c.103.118.22.222.35.31.264.178.683.37 1.285.37.602 0 1.02-.192 1.285-.371.13-.088.247-.192.35-.31l.007-.008a.75.75 0 111.222.87l-.614-.431c.614.43.614.431.613.431v.001l-.001.002-.002.003-.005.007-.014.019a1.984 1.984 0 01-.184.213c-.16.166-.338.316-.53.445-.63.418-1.37.638-2.127.629-.946 0-1.652-.308-2.126-.63a3.32 3.32 0 01-.715-.657l-.014-.02-.005-.006-.002-.003v-.002h-.001l.613-.432-.614.43a.75.75 0 01.183-1.044h.001z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitPullRequestDraft;
impl IconShape for GoGitPullRequestDraft {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.25 1a2.25 2.25 0 00-.75 4.372v5.256a2.251 2.251 0 101.5 0V5.372A2.25 2.25 0 003.25 1zm0 11a.75.75 0 100 1.5.75.75 0 000-1.5zm9.5 3a2.25 2.25 0 100-4.5 2.25 2.25 0 000 4.5zm0-3a.75.75 0 100 1.5.75.75 0 000-1.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M14 7.5a1.25 1.25 0 11-2.5 0 1.25 1.25 0 012.5 0zm0-4.25a1.25 1.25 0 11-2.5 0 1.25 1.25 0 012.5 0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHeading;
impl IconShape for GoHeading {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.75 2a.75.75 0 01.75.75V7h7V2.75a.75.75 0 011.5 0v10.5a.75.75 0 01-1.5 0V8.5h-7v4.75a.75.75 0 01-1.5 0V2.75A.75.75 0 013.75 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoForked;
impl IconShape for GoRepoForked {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm0 2.122a2.25 2.25 0 10-1.5 0v.878A2.25 2.25 0 005.75 8.5h1.5v2.128a2.251 2.251 0 101.5 0V8.5h1.5a2.25 2.25 0 002.25-2.25v-.878a2.25 2.25 0 10-1.5 0v.878a.75.75 0 01-.75.75h-4.5A.75.75 0 015 6.25v-.878zm3.75 7.378a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm3-8.75a.75.75 0 100-1.5.75.75 0 000 1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTelescopeFill;
impl IconShape for GoTelescopeFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.531 10.21a.75.75 0 01.944.253l2.644 3.864a.75.75 0 11-1.238.847L9 12.424v2.826a.75.75 0 01-1.5 0v-2.826l-1.881 2.75a.75.75 0 01-1.238-.848l2.048-2.992a.75.75 0 01.293-.252l1.81-.871zM11.905.42a1.5 1.5 0 012.144.49l1.692 2.93a1.5 1.5 0 01-.649 2.102L2.895 11.815a1.5 1.5 0 01-1.95-.602l-.68-1.176a1.5 1.5 0 01.455-1.99L11.905.422zM3.279 8.119l.835 1.445 1.355-.653-.947-1.64-1.243.848zm7.728-1.874L9.6 3.808l1.243-.848 1.52 2.631-1.356.653z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMegaphone;
impl IconShape for GoMegaphone {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            g {
                fill_rule: "evenodd",
            }
            path {
                d: "M3.25 9a.75.75 0 01.75.75c0 2.142.456 3.828.733 4.653a.121.121 0 00.05.064.207.207 0 00.117.033h1.31c.085 0 .18-.042.258-.152a.448.448 0 00.075-.366A16.74 16.74 0 016 9.75a.75.75 0 011.5 0c0 1.588.25 2.926.494 3.85.293 1.113-.504 2.4-1.783 2.4H4.9c-.686 0-1.35-.41-1.589-1.12A16.42 16.42 0 012.5 9.75.75.75 0 013.25 9z",
            }
            path {
                d: "M0 6a4 4 0 014-4h2.75a.75.75 0 01.75.75v6.5a.75.75 0 01-.75.75H4a4 4 0 01-4-4zm4-2.5a2.5 2.5 0 000 5h2v-5H4z",
            }
            path {
                d: "M15.59.082A.75.75 0 0116 .75v10.5a.75.75 0 01-1.189.608l-.002-.001h.001l-.014-.01a5.829 5.829 0 00-.422-.25 10.58 10.58 0 00-1.469-.64C11.576 10.484 9.536 10 6.75 10a.75.75 0 110-1.5c2.964 0 5.174.516 6.658 1.043.423.151.787.302 1.092.443V2.014c-.305.14-.669.292-1.092.443C11.924 2.984 9.713 3.5 6.75 3.5a.75.75 0 110-1.5c2.786 0 4.826-.484 6.155-.957.665-.236 1.154-.47 1.47-.64a5.82 5.82 0 00.421-.25l.014-.01a.75.75 0 01.78-.061zm-.78.06zm.44 11.108l-.44.607.44-.607z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPackageDependents;
impl IconShape for GoPackageDependents {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.122.392a1.75 1.75 0 011.756 0l5.25 3.045c.54.313.872.89.872 1.514V7.25a.75.75 0 01-1.5 0V5.677L7.75 8.432v6.384a1 1 0 01-1.502.865L.872 12.563A1.75 1.75 0 010 11.049V4.951c0-.624.332-1.2.872-1.514L6.122.392zM7.125 1.69l4.63 2.685L7 7.133 2.245 4.375l4.63-2.685a.25.25 0 01.25 0zM1.5 11.049V5.677l4.75 2.755v5.516l-4.625-2.683a.25.25 0 01-.125-.216zm10.828 3.684a.75.75 0 101.087 1.034l2.378-2.5a.75.75 0 000-1.034l-2.378-2.5a.75.75 0 00-1.087 1.034L13.501 12H10.25a.75.75 0 000 1.5h3.251l-1.173 1.233z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedTrophy;
impl IconShape for GoFeedTrophy {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 5h1v1.146a1 1 0 01-.629.928L11 7.223V5zM5 7.223l-.371-.149A1 1 0 014 6.146V5h1v2.223z",
            }
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zM3 5a1 1 0 011-1h8a1 1 0 011 1v1.146a2 2 0 01-1.257 1.857l-.865.346a3.005 3.005 0 01-2.294 2.094C8.78 11.405 9.342 12 10.5 12a.5.5 0 010 1h-5a.5.5 0 010-1h.002c1.156 0 1.718-.596 1.914-1.557A3.005 3.005 0 015.122 8.35l-.865-.346A2 2 0 013 6.146V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRuby;
impl IconShape for GoRuby {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.637 2.291A.75.75 0 014.23 2h7.54a.75.75 0 01.593.291l3.48 4.5a.75.75 0 01-.072.999l-7.25 7a.75.75 0 01-1.042 0l-7.25-7a.75.75 0 01-.072-.999l3.48-4.5zM4.598 3.5L1.754 7.177 8 13.207l6.246-6.03L11.402 3.5H4.598z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBroadcast;
impl IconShape for GoBroadcast {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.267 1.457c.3.286.312.76.026 1.06A6.475 6.475 0 001.5 7a6.472 6.472 0 001.793 4.483.75.75 0 01-1.086 1.034 8.89 8.89 0 01-.276-.304l.569-.49-.569.49A7.971 7.971 0 010 7c0-2.139.84-4.083 2.207-5.517a.75.75 0 011.06-.026zm9.466 0a.75.75 0 011.06.026A7.975 7.975 0 0116 7c0 2.139-.84 4.083-2.207 5.517a.75.75 0 11-1.086-1.034A6.475 6.475 0 0014.5 7a6.475 6.475 0 00-1.793-4.483.75.75 0 01.026-1.06zM8.75 8.582a1.75 1.75 0 10-1.5 0v5.668a.75.75 0 001.5 0V8.582zM5.331 4.736a.75.75 0 10-1.143-.972A4.983 4.983 0 003 7c0 1.227.443 2.352 1.177 3.222a.75.75 0 001.146-.967A3.483 3.483 0 014.5 7c0-.864.312-1.654.831-2.264zm6.492-.958a.75.75 0 00-1.146.967c.514.61.823 1.395.823 2.255 0 .86-.31 1.646-.823 2.255a.75.75 0 101.146.967A4.983 4.983 0 0013 7a4.983 4.983 0 00-1.177-3.222z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMultiSelect;
impl IconShape for GoMultiSelect {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 2.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5zm4 5a.75.75 0 000 1.5h7.5a.75.75 0 000-1.5h-7.5zm0 5a.75.75 0 000 1.5h7.5a.75.75 0 000-1.5h-7.5zM3 8a1 1 0 11-2 0 1 1 0 012 0zm-1 6a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
            path {
                d: "M13.314 4.918L11.07 2.417A.25.25 0 0111.256 2h4.488a.25.25 0 01.186.417l-2.244 2.5a.25.25 0 01-.372 0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoUnverified;
impl IconShape for GoUnverified {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.415.52a2.678 2.678 0 013.17 0l.928.68c.153.113.33.186.518.215l1.138.175a2.678 2.678 0 012.241 2.24l.175 1.138c.029.187.102.365.215.518l.68.928a2.678 2.678 0 010 3.17l-.68.928a1.179 1.179 0 00-.215.518l-.175 1.138a2.678 2.678 0 01-2.241 2.241l-1.138.175a1.179 1.179 0 00-.518.215l-.928.68a2.678 2.678 0 01-3.17 0l-.928-.68a1.179 1.179 0 00-.518-.215L3.83 14.41a2.678 2.678 0 01-2.24-2.24l-.175-1.138a1.179 1.179 0 00-.215-.518l-.68-.928a2.678 2.678 0 010-3.17l.68-.928a1.17 1.17 0 00.215-.518l.175-1.14a2.678 2.678 0 012.24-2.24l1.138-.175c.187-.029.365-.102.518-.215l.928-.68zm2.282 1.209a1.178 1.178 0 00-1.394 0l-.928.68a2.678 2.678 0 01-1.18.489l-1.136.174a1.178 1.178 0 00-.987.987l-.174 1.137a2.678 2.678 0 01-.489 1.18l-.68.927c-.305.415-.305.98 0 1.394l.68.928c.256.348.423.752.489 1.18l.174 1.136c.078.51.478.909.987.987l1.137.174c.427.066.831.233 1.18.489l.927.68c.415.305.98.305 1.394 0l.928-.68a2.678 2.678 0 011.18-.489l1.136-.174c.51-.078.909-.478.987-.987l.174-1.137c.066-.427.233-.831.489-1.18l.68-.927c.305-.415.305-.98 0-1.394l-.68-.928a2.678 2.678 0 01-.489-1.18l-.174-1.136a1.178 1.178 0 00-.987-.987l-1.137-.174a2.678 2.678 0 01-1.18-.489l-.927-.68zM9 11a1 1 0 11-2 0 1 1 0 012 0zM6.92 6.085c.081-.16.19-.299.34-.398.145-.097.371-.187.74-.187.28 0 .553.087.738.225A.613.613 0 019 6.25c0 .177-.04.264-.077.318a.956.956 0 01-.277.245c-.076.051-.158.1-.258.161l-.007.004c-.093.056-.204.122-.313.195a2.416 2.416 0 00-.692.661.75.75 0 001.248.832.956.956 0 01.276-.245 6.3 6.3 0 01.26-.16l.006-.004c.093-.057.204-.123.313-.195.222-.149.487-.355.692-.662.214-.32.329-.702.329-1.15 0-.76-.36-1.348-.862-1.725A2.76 2.76 0 008 4c-.631 0-1.154.16-1.572.438-.413.276-.68.638-.849.977a.75.75 0 001.342.67z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTerminal;
impl IconShape for GoTerminal {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 2.75C0 1.784.784 1 1.75 1h12.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25V2.75zm1.75-.25a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V2.75a.25.25 0 00-.25-.25H1.75zM7.25 8a.75.75 0 01-.22.53l-2.25 2.25a.75.75 0 11-1.06-1.06L5.44 8 3.72 6.28a.75.75 0 111.06-1.06l2.25 2.25c.141.14.22.331.22.53zm1.5 1.5a.75.75 0 000 1.5h3a.75.75 0 000-1.5h-3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMeter;
impl IconShape for GoMeter {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1.5a6.5 6.5 0 106.016 4.035.75.75 0 011.388-.57 8 8 0 11-4.37-4.37.75.75 0 01-.569 1.389A6.479 6.479 0 008 1.5zm6.28.22a.75.75 0 010 1.06l-4.063 4.064a2.5 2.5 0 11-1.06-1.06L13.22 1.72a.75.75 0 011.06 0zM7 8a1 1 0 112 0 1 1 0 01-2 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoThumbsdown;
impl IconShape for GoThumbsdown {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.083 15.986c1.34.153 2.334-.982 2.334-2.183v-.5c0-1.329.646-2.123 1.317-2.614.329-.24.66-.403.919-.508a1.75 1.75 0 001.514.872h1a1.75 1.75 0 001.75-1.75v-7.5a1.75 1.75 0 00-1.75-1.75h-1a1.75 1.75 0 00-1.662 1.2c-.525-.074-1.068-.228-1.726-.415L9.305.705C8.151.385 6.765.053 4.917.053c-1.706 0-2.97.152-3.722 1.139-.353.463-.537 1.042-.669 1.672C.41 3.424.32 4.108.214 4.897l-.04.306c-.25 1.869-.266 3.318.188 4.316.244.537.622.943 1.136 1.2.495.248 1.066.334 1.669.334h1.422l-.015.112c-.07.518-.157 1.17-.157 1.638 0 .921.151 1.718.655 2.299.512.589 1.248.797 2.011.884zm4.334-13.232c-.706-.089-1.39-.284-2.072-.479a63.914 63.914 0 00-.441-.125c-1.096-.304-2.335-.597-3.987-.597-1.794 0-2.28.222-2.529.548-.147.193-.275.505-.393 1.07-.105.502-.188 1.124-.295 1.93l-.04.3c-.25 1.882-.19 2.933.067 3.497a.921.921 0 00.443.48c.208.104.52.175.997.175h1.75c.685 0 1.295.577 1.205 1.335-.022.192-.049.39-.075.586-.066.488-.13.97-.13 1.329 0 .808.144 1.15.288 1.316.137.157.401.303 1.048.377.307.035.664-.237.664-.693v-.5c0-1.922.978-3.127 1.932-3.825a5.862 5.862 0 011.568-.809V2.754zm1.75 6.798a.25.25 0 01-.25-.25v-7.5a.25.25 0 01.25-.25h1a.25.25 0 01.25.25v7.5a.25.25 0 01-.25.25h-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiffRenamed;
impl IconShape for GoDiffRenamed {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.75 2.5h10.5a.25.25 0 01.25.25v10.5a.25.25 0 01-.25.25H2.75a.25.25 0 01-.25-.25V2.75a.25.25 0 01.25-.25zM13.25 1H2.75A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1zm-1.47 7.53a.75.75 0 000-1.06L8.53 4.22a.75.75 0 00-1.06 1.06l1.97 1.97H4.75a.75.75 0 000 1.5h4.69l-1.97 1.97a.75.75 0 101.06 1.06l3.25-3.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLink;
impl IconShape for GoLink {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShieldX;
impl IconShape for GoShieldX {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.533.133a1.75 1.75 0 00-1.066 0l-5.25 1.68A1.75 1.75 0 001 3.48V7c0 1.566.32 3.182 1.303 4.682.983 1.498 2.585 2.813 5.032 3.855a1.7 1.7 0 001.33 0c2.447-1.042 4.049-2.357 5.032-3.855C14.68 10.182 15 8.566 15 7V3.48a1.75 1.75 0 00-1.217-1.667L8.533.133zm-.61 1.429a.25.25 0 01.153 0l5.25 1.68a.25.25 0 01.174.238V7c0 1.358-.275 2.666-1.057 3.86-.784 1.194-2.121 2.34-4.366 3.297a.2.2 0 01-.154 0c-2.245-.956-3.582-2.104-4.366-3.298C2.775 9.666 2.5 8.36 2.5 7V3.48a.25.25 0 01.174-.237l5.25-1.68zM6.78 5.22a.75.75 0 10-1.06 1.06L6.94 7.5 5.72 8.72a.75.75 0 001.06 1.06L8 8.56l1.22 1.22a.75.75 0 101.06-1.06L9.06 7.5l1.22-1.22a.75.75 0 10-1.06-1.06L8 6.44 6.78 5.22z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLog;
impl IconShape for GoLog {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 8.25a.75.75 0 01.75-.75h4a.75.75 0 010 1.5h-4A.75.75 0 015 8.25zM4 10.5A.75.75 0 004 12h4a.75.75 0 000-1.5H4z",
            }
            path {
                d: "M13-.005H3a3 3 0 00-3 3c0 .676.224 1.254.603 1.722.526.65 1.331.783 1.907.783h1.177c-.364.662-.814 1.339-1.287 2.048-.205.309-.414.624-.623.946C.891 9.865 0 11.418 0 13a3 3 0 003 3h10a3 3 0 001.667-5.494.75.75 0 00-.834 1.246A1.5 1.5 0 1111.5 13c0-.642.225-1.347.623-2.136.397-.787.933-1.593 1.501-2.446l.011-.017c.554-.83 1.139-1.709 1.582-2.588.445-.885.783-1.836.783-2.818 0-1.672-1.346-3-3-3zm-10 1.5a1.5 1.5 0 00-1.5 1.5c0 .321.1.569.27.778.097.12.325.227.74.227h7.674A2.737 2.737 0 0110 2.995c0-.546.146-1.059.401-1.5H3zm10 0c.831 0 1.5.662 1.5 1.5 0 .646-.225 1.353-.623 2.143-.398.79-.933 1.595-1.501 2.448l-.017.026c-.552.828-1.134 1.702-1.575 2.576C10.338 11.072 10 12.021 10 13c0 .546.146 1.059.401 1.5H3A1.5 1.5 0 011.5 13c0-1.084.63-2.289 1.537-3.692.177-.274.366-.556.558-.845.632-.948 1.306-1.96 1.773-2.963h6.382a.75.75 0 00.417-1.373c-.444-.298-.667-.656-.667-1.132a1.5 1.5 0 011.5-1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBell;
impl IconShape for GoBell {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z",
            }
            path {
                d: "M8 1.5A3.5 3.5 0 004.5 5v2.947c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01l.001.006c0 .002.002.004.004.006a.017.017 0 00.006.004l.007.001h10.964l.007-.001a.016.016 0 00.006-.004.016.016 0 00.004-.006l.001-.007a.017.017 0 00-.003-.01l-1.703-2.554a1.75 1.75 0 01-.294-.97V5A3.5 3.5 0 008 1.5zM3 5a5 5 0 0110 0v2.947c0 .05.015.098.042.139l1.703 2.555A1.518 1.518 0 0113.482 13H2.518a1.518 1.518 0 01-1.263-2.36l1.703-2.554A.25.25 0 003 7.947V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGlobe;
impl IconShape for GoGlobe {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.543 7.25h2.733c.144-2.074.866-3.756 1.58-4.948.12-.197.237-.381.353-.552a6.506 6.506 0 00-4.666 5.5zm2.733 1.5H1.543a6.506 6.506 0 004.666 5.5 11.13 11.13 0 01-.352-.552c-.715-1.192-1.437-2.874-1.581-4.948zm1.504 0h4.44a9.637 9.637 0 01-1.363 4.177c-.306.51-.612.919-.857 1.215a9.978 9.978 0 01-.857-1.215A9.637 9.637 0 015.78 8.75zm4.44-1.5H5.78a9.637 9.637 0 011.363-4.177c.306-.51.612-.919.857-1.215.245.296.55.705.857 1.215A9.638 9.638 0 0110.22 7.25zm1.504 1.5c-.144 2.074-.866 3.756-1.58 4.948-.12.197-.237.381-.353.552a6.506 6.506 0 004.666-5.5h-2.733zm2.733-1.5h-2.733c-.144-2.074-.866-3.756-1.58-4.948a11.738 11.738 0 00-.353-.552 6.506 6.506 0 014.666 5.5zM8 0a8 8 0 100 16A8 8 0 008 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFoldUp;
impl IconShape for GoFoldUp {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.823 1.677L4.927 4.573A.25.25 0 005.104 5H7.25v3.236a.75.75 0 101.5 0V5h2.146a.25.25 0 00.177-.427L8.177 1.677a.25.25 0 00-.354 0zM13.75 11a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zm-3.75.75a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5a.75.75 0 01-.75-.75zM7.75 11a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM4 11.75a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5a.75.75 0 01-.75-.75zM1.75 11a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoXCircle;
impl IconShape for GoXCircle {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.404 12.596a6.5 6.5 0 119.192-9.192 6.5 6.5 0 01-9.192 9.192zM2.344 2.343a8 8 0 1011.313 11.314A8 8 0 002.343 2.343zM6.03 4.97a.75.75 0 00-1.06 1.06L6.94 8 4.97 9.97a.75.75 0 101.06 1.06L8 9.06l1.97 1.97a.75.75 0 101.06-1.06L9.06 8l1.97-1.97a.75.75 0 10-1.06-1.06L8 6.94 6.03 4.97z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPlusCircle;
impl IconShape for GoPlusCircle {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zm.75 4.75a.75.75 0 00-1.5 0v2.5h-2.5a.75.75 0 000 1.5h2.5v2.5a.75.75 0 001.5 0v-2.5h2.5a.75.75 0 000-1.5h-2.5v-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDependabot;
impl IconShape for GoDependabot {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.75 7.5a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5a.75.75 0 01.75-.75zm5.25.75a.75.75 0 00-1.5 0v1.5a.75.75 0 001.5 0v-1.5z",
            }
            path {
                d: "M6.25 0a.75.75 0 000 1.5H7.5v2H3.75A2.25 2.25 0 001.5 5.75V8H.75a.75.75 0 000 1.5h.75v2.75a2.25 2.25 0 002.25 2.25h8.5a2.25 2.25 0 002.25-2.25V9.5h.75a.75.75 0 000-1.5h-.75V5.75a2.25 2.25 0 00-2.25-2.25H9V.75A.75.75 0 008.25 0h-2zM3 5.75A.75.75 0 013.75 5h8.5a.75.75 0 01.75.75v6.5a.75.75 0 01-.75.75h-8.5a.75.75 0 01-.75-.75v-6.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTriangleLeft;
impl IconShape for GoTriangleLeft {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.573 4.427L6.177 7.823a.25.25 0 000 .354l3.396 3.396a.25.25 0 00.427-.177V4.604a.25.25 0 00-.427-.177z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoApps;
impl IconShape for GoApps {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 3.25c0-.966.784-1.75 1.75-1.75h2.5c.966 0 1.75.784 1.75 1.75v2.5A1.75 1.75 0 015.75 7.5h-2.5A1.75 1.75 0 011.5 5.75v-2.5zM3.25 3a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h2.5A.25.25 0 006 5.75v-2.5A.25.25 0 005.75 3h-2.5zM1.5 10.25c0-.966.784-1.75 1.75-1.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a1.75 1.75 0 01-1.75 1.75h-2.5a1.75 1.75 0 01-1.75-1.75v-2.5zM3.25 10a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h2.5a.25.25 0 00.25-.25v-2.5a.25.25 0 00-.25-.25h-2.5zM8.5 3.25c0-.966.784-1.75 1.75-1.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a1.75 1.75 0 01-1.75 1.75h-2.5A1.75 1.75 0 018.5 5.75v-2.5zM10.25 3a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h2.5a.25.25 0 00.25-.25v-2.5a.25.25 0 00-.25-.25h-2.5zM8.5 10.25c0-.966.784-1.75 1.75-1.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a1.75 1.75 0 01-1.75 1.75h-2.5a1.75 1.75 0 01-1.75-1.75v-2.5zm1.75-.25a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h2.5a.25.25 0 00.25-.25v-2.5a.25.25 0 00-.25-.25h-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoNorthStar;
impl IconShape for GoNorthStar {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.5.75a.75.75 0 00-1.5 0v5.19L4.391 3.33a.75.75 0 10-1.06 1.061L5.939 7H.75a.75.75 0 000 1.5h5.19l-2.61 2.609a.75.75 0 101.061 1.06L7 9.561v5.189a.75.75 0 001.5 0V9.56l2.609 2.61a.75.75 0 101.06-1.061L9.561 8.5h5.189a.75.75 0 000-1.5H9.56l2.61-2.609a.75.75 0 00-1.061-1.06L8.5 5.939V.75z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCrossReference;
impl IconShape for GoCrossReference {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 1.25v4.146a.25.25 0 01-.427.177L14.03 4.03l-3.75 3.75a.75.75 0 11-1.06-1.06l3.75-3.75-1.543-1.543A.25.25 0 0111.604 1h4.146a.25.25 0 01.25.25zM2.75 3.5a.25.25 0 00-.25.25v7.5c0 .138.112.25.25.25h2a.75.75 0 01.75.75v2.19l2.72-2.72a.75.75 0 01.53-.22h4.5a.25.25 0 00.25-.25v-2.5a.75.75 0 111.5 0v2.5A1.75 1.75 0 0113.25 13H9.06l-2.573 2.573A1.457 1.457 0 014 14.543V13H2.75A1.75 1.75 0 011 11.25v-7.5C1 2.784 1.784 2 2.75 2h5.5a.75.75 0 010 1.5h-5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIssueOpened;
impl IconShape for GoIssueOpened {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 9.5a1.5 1.5 0 100-3 1.5 1.5 0 000 3z",
            }
            path {
                d: "M8 0a8 8 0 100 16A8 8 0 008 0zM1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIterations;
impl IconShape for GoIterations {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.5 7.25a4.75 4.75 0 019.5 0 .75.75 0 001.5 0 6.25 6.25 0 10-6.25 6.25H12v2.146c0 .223.27.335.427.177l2.896-2.896a.25.25 0 000-.354l-2.896-2.896a.25.25 0 00-.427.177V12H7.25A4.75 4.75 0 012.5 7.25z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTabExternal;
impl IconShape for GoTabExternal {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.25 4a.25.25 0 00-.25.25v9a.75.75 0 01-.75.75H.75a.75.75 0 010-1.5h.75V4.25c0-.966.784-1.75 1.75-1.75h9.5c.966 0 1.75.784 1.75 1.75v8.25h.75a.75.75 0 010 1.5h-1.5a.75.75 0 01-.75-.75v-9a.25.25 0 00-.25-.25h-9.5z",
            }
            path {
                d: "M7.97 7.97l-2.75 2.75a.75.75 0 101.06 1.06l2.75-2.75 1.543 1.543a.25.25 0 00.427-.177V6.25a.25.25 0 00-.25-.25H6.604a.25.25 0 00-.177.427L7.97 7.97z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedTag;
impl IconShape for GoFeedTag {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.22 6.5a.72.72 0 11-1.44 0 .72.72 0 011.44 0z",
            }
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zM4 8.379V5a1 1 0 011-1h3.379a1.5 1.5 0 011.06.44l3.213 3.211a1.2 1.2 0 010 1.698l-3.303 3.303a1.2 1.2 0 01-1.698 0L4.44 9.439A1.5 1.5 0 014 8.38z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoKeyAsterisk;
impl IconShape for GoKeyAsterisk {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 2.75A2.75 2.75 0 012.75 0h10.5A2.75 2.75 0 0116 2.75v10.5A2.75 2.75 0 0113.25 16H2.75A2.75 2.75 0 010 13.25V2.75zM2.75 1.5c-.69 0-1.25.56-1.25 1.25v10.5c0 .69.56 1.25 1.25 1.25h10.5c.69 0 1.25-.56 1.25-1.25V2.75c0-.69-.56-1.25-1.25-1.25H2.75z",
                fill_rule: "evenodd",
            }
            path {
                d: "M8 4a.75.75 0 01.75.75V6.7l1.69-.975a.75.75 0 01.75 1.3L9.5 8l1.69.976a.75.75 0 01-.75 1.298L8.75 9.3v1.951a.75.75 0 01-1.5 0V9.299l-1.69.976a.75.75 0 01-.75-1.3L6.5 8l-1.69-.975a.75.75 0 01.75-1.3l1.69.976V4.75A.75.75 0 018 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTrophy;
impl IconShape for GoTrophy {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.217 6.962A3.75 3.75 0 010 3.25v-.5C0 1.784.784 1 1.75 1h1.356c.228-.585.796-1 1.462-1h6.864a1.57 1.57 0 011.462 1h1.356c.966 0 1.75.784 1.75 1.75v.5a3.75 3.75 0 01-3.217 3.712 5.014 5.014 0 01-2.771 3.117l.144 1.446c.005.05.03.12.114.204.086.087.217.17.373.227.283.103.618.274.89.568.285.31.467.723.467 1.226v.75h1.25a.75.75 0 110 1.5H2.75a.75.75 0 010-1.5H4v-.75c0-.503.182-.916.468-1.226.27-.294.606-.465.889-.568a1.03 1.03 0 00.373-.227c.084-.085.109-.153.114-.204l.144-1.446a5.014 5.014 0 01-2.77-3.117zM3 2.5H1.75a.25.25 0 00-.25.25v.5c0 .98.626 1.813 1.5 2.122V2.5zm4.457 7.97l-.12 1.204c-.093.925-.858 1.47-1.467 1.691a.764.764 0 00-.3.176c-.037.04-.07.093-.07.21v.75h5v-.75c0-.117-.033-.17-.07-.21a.763.763 0 00-.3-.176c-.609-.221-1.374-.766-1.466-1.69l-.12-1.204a5.052 5.052 0 01-1.087 0zM13 5.373V2.5h1.25a.25.25 0 01.25.25v.5A2.25 2.25 0 0113 5.372zM4.5 1.568c0-.037.03-.068.068-.068h6.864c.037 0 .068.03.068.068V5.5a3.5 3.5 0 11-7 0V1.568z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoInfinity;
impl IconShape for GoInfinity {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.5 6c-1.086 0-2 .914-2 2 0 1.086.914 2 2 2 .525 0 1.122-.244 1.825-.727.51-.35 1.025-.79 1.561-1.273-.536-.483-1.052-.922-1.56-1.273C4.621 6.244 4.025 6 3.5 6zm4.5.984c-.59-.533-1.204-1.066-1.825-1.493-.797-.548-1.7-.991-2.675-.991C1.586 4.5 0 6.086 0 8s1.586 3.5 3.5 3.5c.975 0 1.878-.444 2.675-.991.621-.427 1.235-.96 1.825-1.493.59.533 1.204 1.066 1.825 1.493.797.547 1.7.991 2.675.991 1.914 0 3.5-1.586 3.5-3.5s-1.586-3.5-3.5-3.5c-.975 0-1.878.443-2.675.991-.621.427-1.235.96-1.825 1.493zM9.114 8c.536.483 1.052.922 1.56 1.273.704.483 1.3.727 1.826.727 1.086 0 2-.914 2-2 0-1.086-.914-2-2-2-.525 0-1.122.244-1.825.727-.51.35-1.025.79-1.561 1.273z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTypography;
impl IconShape for GoTypography {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.21 8.5L4.574 3.594 2.857 8.5H6.21zm.5 1.5l.829 2.487a.75.75 0 001.423-.474L5.735 2.332a1.216 1.216 0 00-2.302-.018l-3.39 9.688a.75.75 0 001.415.496L2.332 10H6.71zm3.13-4.358C10.53 4.374 11.87 4 13 4c1.5 0 3 .939 3 2.601v5.649a.75.75 0 01-1.448.275C13.995 12.82 13.3 13 12.5 13c-.77 0-1.514-.231-2.078-.709-.577-.488-.922-1.199-.922-2.041 0-.694.265-1.411.887-1.944C11 7.78 11.88 7.5 13 7.5h1.5v-.899c0-.54-.5-1.101-1.5-1.101-.869 0-1.528.282-1.84.858a.75.75 0 11-1.32-.716zM14.5 9H13c-.881 0-1.375.22-1.637.444-.253.217-.363.5-.363.806 0 .408.155.697.39.896.249.21.63.354 1.11.354.732 0 1.26-.209 1.588-.449.35-.257.412-.495.412-.551V9z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileRemoved;
impl IconShape for GoFileRemoved {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.75 1.5a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 00.25-.25V4.664a.25.25 0 00-.073-.177l-2.914-2.914a.25.25 0 00-.177-.073H3.75zM2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0113.25 16h-9.5A1.75 1.75 0 012 14.25V1.75zM8.25 7.5h2.242a.75.75 0 010 1.5h-2.24l-2.254.015a.75.75 0 01-.01-1.5L8.25 7.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStrikethrough;
impl IconShape for GoStrikethrough {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.581 3.25c-2.036 0-2.778 1.082-2.778 1.786 0 .055.002.107.006.157a.75.75 0 01-1.496.114 3.56 3.56 0 01-.01-.271c0-1.832 1.75-3.286 4.278-3.286 1.418 0 2.721.58 3.514 1.093a.75.75 0 11-.814 1.26c-.64-.414-1.662-.853-2.7-.853zm3.474 5.25h3.195a.75.75 0 000-1.5H1.75a.75.75 0 000 1.5h6.018c.835.187 1.503.464 1.951.81.439.34.647.725.647 1.197 0 .428-.159.895-.594 1.267-.444.38-1.254.726-2.676.726-1.373 0-2.38-.493-2.86-.956a.75.75 0 00-1.042 1.079C3.992 13.393 5.39 14 7.096 14c1.652 0 2.852-.403 3.65-1.085a3.134 3.134 0 001.12-2.408 2.85 2.85 0 00-.811-2.007z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMilestone;
impl IconShape for GoMilestone {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.75 0a.75.75 0 01.75.75V3h3.634c.414 0 .814.147 1.13.414l2.07 1.75a1.75 1.75 0 010 2.672l-2.07 1.75a1.75 1.75 0 01-1.13.414H8.5v5.25a.75.75 0 11-1.5 0V10H2.75A1.75 1.75 0 011 8.25v-3.5C1 3.784 1.784 3 2.75 3H7V.75A.75.75 0 017.75 0zm0 8.5h4.384a.25.25 0 00.161-.06l2.07-1.75a.25.25 0 000-.38l-2.07-1.75a.25.25 0 00-.161-.06H2.75a.25.25 0 00-.25.25v3.5c0 .138.112.25.25.25h5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHeart;
impl IconShape for GoHeart {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.25 2.5c-1.336 0-2.75 1.164-2.75 3 0 2.15 1.58 4.144 3.365 5.682A20.565 20.565 0 008 13.393a20.561 20.561 0 003.135-2.211C12.92 9.644 14.5 7.65 14.5 5.5c0-1.836-1.414-3-2.75-3-1.373 0-2.609.986-3.029 2.456a.75.75 0 01-1.442 0C6.859 3.486 5.623 2.5 4.25 2.5zM8 14.25l-.345.666-.002-.001-.006-.003-.018-.01a7.643 7.643 0 01-.31-.17 22.075 22.075 0 01-3.434-2.414C2.045 10.731 0 8.35 0 5.5 0 2.836 2.086 1 4.25 1 5.797 1 7.153 1.802 8 3.02 8.847 1.802 10.203 1 11.75 1 13.914 1 16 2.836 16 5.5c0 2.85-2.045 5.231-3.885 6.818a22.08 22.08 0 01-3.744 2.584l-.018.01-.006.003h-.002L8 14.25zm0 0l.345.666a.752.752 0 01-.69 0L8 14.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileMoved;
impl IconShape for GoFileMoved {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0113.25 16h-3.5a.75.75 0 010-1.5h3.5a.25.25 0 00.25-.25V4.664a.25.25 0 00-.073-.177l-2.914-2.914a.25.25 0 00-.177-.073H3.75a.25.25 0 00-.25.25v6.5a.75.75 0 01-1.5 0v-6.5z",
            }
            path {
                d: "M5.427 15.573l3.146-3.146a.25.25 0 000-.354L5.427 8.927A.25.25 0 005 9.104V11.5H.75a.75.75 0 000 1.5H5v2.396c0 .223.27.335.427.177z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTriangleDown;
impl IconShape for GoTriangleDown {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.427 7.427l3.396 3.396a.25.25 0 00.354 0l3.396-3.396A.25.25 0 0011.396 7H4.604a.25.25 0 00-.177.427z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitMerge;
impl IconShape for GoGitMerge {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 3.254V3.25v.005a.75.75 0 110-.005v.004zm.45 1.9a2.25 2.25 0 10-1.95.218v5.256a2.25 2.25 0 101.5 0V7.123A5.735 5.735 0 009.25 9h1.378a2.251 2.251 0 100-1.5H9.25a4.25 4.25 0 01-3.8-2.346zM12.75 9a.75.75 0 100-1.5.75.75 0 000 1.5zm-8.5 4.5a.75.75 0 100-1.5.75.75 0 000 1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRss;
impl IconShape for GoRss {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.002 2.725a.75.75 0 01.797-.699C8.79 2.42 13.58 7.21 13.974 13.201a.75.75 0 11-1.497.098 10.502 10.502 0 00-9.776-9.776.75.75 0 01-.7-.798zM2 13a1 1 0 112 0 1 1 0 01-2 0zm.84-5.95a.75.75 0 00-.179 1.489c2.509.3 4.5 2.291 4.8 4.8a.75.75 0 101.49-.178A7.003 7.003 0 002.838 7.05z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoChevronUp;
impl IconShape for GoChevronUp {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.22 9.78a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 0l4.25 4.25a.75.75 0 01-1.06 1.06L8 6.06 4.28 9.78a.75.75 0 01-1.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMortarBoard;
impl IconShape for GoMortarBoard {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.693 1.066a.75.75 0 01.614 0l7.25 3.25a.75.75 0 010 1.368L13 6.831v2.794c0 1.024-.81 1.749-1.66 2.173-.893.447-2.075.702-3.34.702-.278 0-.55-.012-.816-.036a.75.75 0 01.133-1.494c.22.02.45.03.683.03 1.082 0 2.025-.221 2.67-.543.69-.345.83-.682.83-.832V7.503L8.307 8.934a.75.75 0 01-.614 0L4 7.28v1.663c.296.105.575.275.812.512.438.438.688 1.059.688 1.796v3a.75.75 0 01-.75.75h-3a.75.75 0 01-.75-.75v-3c0-.737.25-1.358.688-1.796.237-.237.516-.407.812-.512V6.606L.443 5.684a.75.75 0 010-1.368l7.25-3.25zM2.583 5L8 7.428 13.416 5 8 2.572 2.583 5zM2.5 11.25c0-.388.125-.611.25-.735a.704.704 0 01.5-.203c.19 0 .37.071.5.203.125.124.25.347.25.735v2.25H2.5v-2.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSortDesc;
impl IconShape for GoSortDesc {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 4.25a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5H.75A.75.75 0 010 4.25zm0 4a.75.75 0 01.75-.75h4.5a.75.75 0 010 1.5H.75A.75.75 0 010 8.25zm0 4a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5H.75a.75.75 0 01-.75-.75z",
                fill_rule: "evenodd",
            }
            path {
                d: "M13.5 10h2.25a.25.25 0 01.177.427l-3 3a.25.25 0 01-.354 0l-3-3A.25.25 0 019.75 10H12V3.75a.75.75 0 011.5 0V10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCheck;
impl IconShape for GoCheck {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFile;
impl IconShape for GoFile {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.75 1.5a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 00.25-.25V6h-2.75A1.75 1.75 0 019 4.25V1.5H3.75zm6.75.062V4.25c0 .138.112.25.25.25h2.688a.252.252 0 00-.011-.013l-2.914-2.914a.272.272 0 00-.013-.011zM2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0113.25 16h-9.5A1.75 1.75 0 012 14.25V1.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowSwitch;
impl IconShape for GoArrowSwitch {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.22 14.78a.75.75 0 001.06-1.06L4.56 12h8.69a.75.75 0 000-1.5H4.56l1.72-1.72a.75.75 0 00-1.06-1.06l-3 3a.75.75 0 000 1.06l3 3zm5.56-6.5a.75.75 0 11-1.06-1.06l1.72-1.72H2.75a.75.75 0 010-1.5h8.69L9.72 2.28a.75.75 0 011.06-1.06l3 3a.75.75 0 010 1.06l-3 3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowUp;
impl IconShape for GoArrowUp {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.47 7.78a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 0l4.25 4.25a.75.75 0 01-1.06 1.06L9 4.81v7.44a.75.75 0 01-1.5 0V4.81L4.53 7.78a.75.75 0 01-1.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoQuote;
impl IconShape for GoQuote {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 2.5a.75.75 0 000 1.5h10.5a.75.75 0 000-1.5H1.75zm4 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zM2.5 7.75a.75.75 0 00-1.5 0v6a.75.75 0 001.5 0v-6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoInbox;
impl IconShape for GoInbox {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.8 2.06A1.75 1.75 0 014.41 1h7.18c.7 0 1.333.417 1.61 1.06l2.74 6.395a.75.75 0 01.06.295v4.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25v-4.5a.75.75 0 01.06-.295L2.8 2.06zm1.61.44a.25.25 0 00-.23.152L1.887 8H4.75a.75.75 0 01.6.3L6.625 10h2.75l1.275-1.7a.75.75 0 01.6-.3h2.863L11.82 2.652a.25.25 0 00-.23-.152H4.41zm10.09 7h-2.875l-1.275 1.7a.75.75 0 01-.6.3h-3.5a.75.75 0 01-.6-.3L4.375 9.5H1.5v3.75c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V9.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPackage;
impl IconShape for GoPackage {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.878.392a1.75 1.75 0 00-1.756 0l-5.25 3.045A1.75 1.75 0 001 4.951v6.098c0 .624.332 1.2.872 1.514l5.25 3.045a1.75 1.75 0 001.756 0l5.25-3.045c.54-.313.872-.89.872-1.514V4.951c0-.624-.332-1.2-.872-1.514L8.878.392zM7.875 1.69a.25.25 0 01.25 0l4.63 2.685L8 7.133 3.245 4.375l4.63-2.685zM2.5 5.677v5.372c0 .09.047.171.125.216l4.625 2.683V8.432L2.5 5.677zm6.25 8.271l4.625-2.683a.25.25 0 00.125-.216V5.677L8.75 8.432v5.516z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMarkdown;
impl IconShape for GoMarkdown {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.85 3H1.15C.52 3 0 3.52 0 4.15v7.69C0 12.48.52 13 1.15 13h13.69c.64 0 1.15-.52 1.15-1.15v-7.7C16 3.52 15.48 3 14.85 3zM9 11H7V8L5.5 9.92 4 8v3H2V5h2l1.5 2L7 5h2v6zm2.99.5L9.5 8H11V5h2v3h1.5l-2.51 3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDownload;
impl IconShape for GoDownload {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.47 10.78a.75.75 0 001.06 0l3.75-3.75a.75.75 0 00-1.06-1.06L8.75 8.44V1.75a.75.75 0 00-1.5 0v6.69L4.78 5.97a.75.75 0 00-1.06 1.06l3.75 3.75zM3.75 13a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSortAsc;
impl IconShape for GoSortAsc {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 4.25a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5H.75A.75.75 0 010 4.25zm0 4a.75.75 0 01.75-.75h4.5a.75.75 0 010 1.5H.75A.75.75 0 010 8.25zm0 4a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5H.75a.75.75 0 01-.75-.75zm12.927-9.677a.25.25 0 00-.354 0l-3 3A.25.25 0 009.75 6H12v6.75a.75.75 0 001.5 0V6h2.25a.25.25 0 00.177-.427l-3-3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFold;
impl IconShape for GoFold {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.896 2H8.75V.75a.75.75 0 00-1.5 0V2H5.104a.25.25 0 00-.177.427l2.896 2.896a.25.25 0 00.354 0l2.896-2.896A.25.25 0 0010.896 2zM8.75 15.25a.75.75 0 01-1.5 0V14H5.104a.25.25 0 01-.177-.427l2.896-2.896a.25.25 0 01.354 0l2.896 2.896a.25.25 0 01-.177.427H8.75v1.25zm-6.5-6.5a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM6 8a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5A.75.75 0 016 8zm2.25.75a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM12 8a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5A.75.75 0 0112 8zm2.25.75a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoUpload;
impl IconShape for GoUpload {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.53 1.22a.75.75 0 00-1.06 0L3.72 4.97a.75.75 0 001.06 1.06l2.47-2.47v6.69a.75.75 0 001.5 0V3.56l2.47 2.47a.75.75 0 101.06-1.06L8.53 1.22zM3.75 13a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFlame;
impl IconShape for GoFlame {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.998 14.5c2.832 0 5-1.98 5-4.5 0-1.463-.68-2.19-1.879-3.383l-.036-.037c-1.013-1.008-2.3-2.29-2.834-4.434-.322.256-.63.579-.864.953-.432.696-.621 1.58-.046 2.73.473.947.67 2.284-.278 3.232-.61.61-1.545.84-2.403.633a2.788 2.788 0 01-1.436-.874A3.21 3.21 0 003 10c0 2.53 2.164 4.5 4.998 4.5zM9.533.753C9.496.34 9.16.009 8.77.146 7.035.75 4.34 3.187 5.997 6.5c.344.689.285 1.218.003 1.5-.419.419-1.54.487-2.04-.832-.173-.454-.659-.762-1.035-.454C2.036 7.44 1.5 8.702 1.5 10c0 3.512 2.998 6 6.498 6s6.5-2.5 6.5-6c0-2.137-1.128-3.26-2.312-4.438-1.19-1.184-2.436-2.425-2.653-4.81z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGift;
impl IconShape for GoGift {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.75 1.5a1.25 1.25 0 100 2.5h2.309c-.233-.818-.542-1.401-.878-1.793-.43-.502-.915-.707-1.431-.707zM2 2.75c0 .45.108.875.3 1.25h-.55A1.75 1.75 0 000 5.75v2c0 .698.409 1.3 1 1.582v4.918c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 14.25V9.332c.591-.281 1-.884 1-1.582v-2A1.75 1.75 0 0014.25 4h-.55a2.75 2.75 0 00-2.45-4c-.984 0-1.874.42-2.57 1.23A5.086 5.086 0 008 2.274a5.086 5.086 0 00-.68-1.042C6.623.42 5.733 0 4.75 0A2.75 2.75 0 002 2.75zM8.941 4h2.309a1.25 1.25 0 100-2.5c-.516 0-1 .205-1.43.707-.337.392-.646.975-.879 1.793zm-1.84 1.5H1.75a.25.25 0 00-.25.25v2c0 .138.112.25.25.25h5.5V5.5h-.149zm1.649 0V8h5.5a.25.25 0 00.25-.25v-2a.25.25 0 00-.25-.25h-5.5zm0 4h4.75v4.75a.25.25 0 01-.25.25h-4.5v-5zm-1.5 0v5h-4.5a.25.25 0 01-.25-.25V9.5h4.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedRepo;
impl IconShape for GoFeedRepo {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zM5.5 4A1.5 1.5 0 004 5.5v5c0 .828.5 1.5 1 1.5v-1a1 1 0 011-1h5v1h-1v1h1.5a.5.5 0 00.5-.5v-7a.5.5 0 00-.5-.5h-6zm.5 7.25a.25.25 0 01.25-.25H9v2.764a.25.25 0 01-.426.178l-.898-.888a.25.25 0 00-.352 0l-.898.888A.25.25 0 016 13.764V11.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoOrganization;
impl IconShape for GoOrganization {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 14.25c0 .138.112.25.25.25H4v-1.25a.75.75 0 01.75-.75h2.5a.75.75 0 01.75.75v1.25h2.25a.25.25 0 00.25-.25V1.75a.25.25 0 00-.25-.25h-8.5a.25.25 0 00-.25.25v12.5zM1.75 16A1.75 1.75 0 010 14.25V1.75C0 .784.784 0 1.75 0h8.5C11.216 0 12 .784 12 1.75v12.5c0 .085-.006.168-.018.25h2.268a.25.25 0 00.25-.25V8.285a.25.25 0 00-.111-.208l-1.055-.703a.75.75 0 11.832-1.248l1.055.703c.487.325.779.871.779 1.456v5.965A1.75 1.75 0 0114.25 16h-3.5a.75.75 0 01-.197-.026c-.099.017-.2.026-.303.026h-3a.75.75 0 01-.75-.75V14h-1v1.25a.75.75 0 01-.75.75h-3zM3 3.75A.75.75 0 013.75 3h.5a.75.75 0 010 1.5h-.5A.75.75 0 013 3.75zM3.75 6a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM3 9.75A.75.75 0 013.75 9h.5a.75.75 0 010 1.5h-.5A.75.75 0 013 9.75zM7.75 9a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM7 6.75A.75.75 0 017.75 6h.5a.75.75 0 010 1.5h-.5A.75.75 0 017 6.75zM7.75 3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStar;
impl IconShape for GoStar {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCpu;
impl IconShape for GoCpu {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.5.75a.75.75 0 00-1.5 0V2H3.75A1.75 1.75 0 002 3.75V5H.75a.75.75 0 000 1.5H2v3H.75a.75.75 0 000 1.5H2v1.25c0 .966.784 1.75 1.75 1.75H5v1.25a.75.75 0 001.5 0V14h3v1.25a.75.75 0 001.5 0V14h1.25A1.75 1.75 0 0014 12.25V11h1.25a.75.75 0 000-1.5H14v-3h1.25a.75.75 0 000-1.5H14V3.75A1.75 1.75 0 0012.25 2H11V.75a.75.75 0 00-1.5 0V2h-3V.75zm5.75 11.75h-8.5a.25.25 0 01-.25-.25v-8.5a.25.25 0 01.25-.25h8.5a.25.25 0 01.25.25v8.5a.25.25 0 01-.25.25zM5.75 5a.75.75 0 00-.75.75v4.5c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-4.5a.75.75 0 00-.75-.75h-4.5zm.75 4.5v-3h3v3h-3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPersonFill;
impl IconShape for GoPersonFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.243 4.757a3.757 3.757 0 115.851 3.119 6.006 6.006 0 013.9 5.339.75.75 0 01-.715.784H2.721a.75.75 0 01-.714-.784 6.006 6.006 0 013.9-5.34 3.753 3.753 0 01-1.664-3.118z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDash;
impl IconShape for GoDash {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 7.75A.75.75 0 012.75 7h10a.75.75 0 010 1.5h-10A.75.75 0 012 7.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoLocked;
impl IconShape for GoRepoLocked {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 2.5A2.5 2.5 0 013.5 0h8.75a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0V1.5h-8a1 1 0 00-1 1v6.708A2.492 2.492 0 013.5 9h2.75a.75.75 0 010 1.5H3.5a1 1 0 100 2h2.75a.75.75 0 010 1.5H3.5A2.5 2.5 0 011 11.5v-9z",
            }
            path {
                d: "M9 10.168V9a3 3 0 116 0v1.168c.591.281 1 .884 1 1.582v2.5A1.75 1.75 0 0114.25 16h-4.5A1.75 1.75 0 018 14.25v-2.5c0-.698.409-1.3 1-1.582zM13.5 10h-3V9a1.5 1.5 0 013 0v1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCopy;
impl IconShape for GoCopy {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 6.75C0 5.784.784 5 1.75 5h1.5a.75.75 0 010 1.5h-1.5a.25.25 0 00-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 00.25-.25v-1.5a.75.75 0 011.5 0v1.5A1.75 1.75 0 019.25 16h-7.5A1.75 1.75 0 010 14.25v-7.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M5 1.75C5 .784 5.784 0 6.75 0h7.5C15.216 0 16 .784 16 1.75v7.5A1.75 1.75 0 0114.25 11h-7.5A1.75 1.75 0 015 9.25v-7.5zm1.75-.25a.25.25 0 00-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 00.25-.25v-7.5a.25.25 0 00-.25-.25h-7.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTag;
impl IconShape for GoTag {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.5 7.775V2.75a.25.25 0 01.25-.25h5.025a.25.25 0 01.177.073l6.25 6.25a.25.25 0 010 .354l-5.025 5.025a.25.25 0 01-.354 0l-6.25-6.25a.25.25 0 01-.073-.177zm-1.5 0V2.75C1 1.784 1.784 1 2.75 1h5.025c.464 0 .91.184 1.238.513l6.25 6.25a1.75 1.75 0 010 2.474l-5.026 5.026a1.75 1.75 0 01-2.474 0l-6.25-6.25A1.75 1.75 0 011 7.775zM6 5a1 1 0 100 2 1 1 0 000-2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPackageDependencies;
impl IconShape for GoPackageDependencies {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.122.392a1.75 1.75 0 011.756 0l5.25 3.045c.54.313.872.89.872 1.514V7.25a.75.75 0 01-1.5 0V5.677L7.75 8.432v6.384a1 1 0 01-1.502.865L.872 12.563A1.75 1.75 0 010 11.049V4.951c0-.624.332-1.2.872-1.514L6.122.392zM7.125 1.69l4.63 2.685L7 7.133 2.245 4.375l4.63-2.685a.25.25 0 01.25 0zM1.5 11.049V5.677l4.75 2.755v5.516l-4.625-2.683a.25.25 0 01-.125-.216zm11.672-.282a.75.75 0 10-1.087-1.034l-2.378 2.5a.75.75 0 000 1.034l2.378 2.5a.75.75 0 101.087-1.034L11.999 13.5h3.251a.75.75 0 000-1.5h-3.251l1.173-1.233z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShieldLock;
impl IconShape for GoShieldLock {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.533.133a1.75 1.75 0 00-1.066 0l-5.25 1.68A1.75 1.75 0 001 3.48V7c0 1.566.32 3.182 1.303 4.682.983 1.498 2.585 2.813 5.032 3.855a1.7 1.7 0 001.33 0c2.447-1.042 4.049-2.357 5.032-3.855C14.68 10.182 15 8.566 15 7V3.48a1.75 1.75 0 00-1.217-1.667L8.533.133zm-.61 1.429a.25.25 0 01.153 0l5.25 1.68a.25.25 0 01.174.238V7c0 1.358-.275 2.666-1.057 3.86-.784 1.194-2.121 2.34-4.366 3.297a.2.2 0 01-.154 0c-2.245-.956-3.582-2.104-4.366-3.298C2.775 9.666 2.5 8.36 2.5 7V3.48a.25.25 0 01.174-.237l5.25-1.68zM9.5 6.5a1.5 1.5 0 01-.75 1.3v2.45a.75.75 0 01-1.5 0V7.8A1.5 1.5 0 119.5 6.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoScreenNormal;
impl IconShape for GoScreenNormal {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.25 1a.75.75 0 01.75.75v2.5A1.75 1.75 0 014.25 6h-2.5a.75.75 0 010-1.5h2.5a.25.25 0 00.25-.25v-2.5A.75.75 0 015.25 1zm5.5 0a.75.75 0 01.75.75v2.5c0 .138.112.25.25.25h2.5a.75.75 0 010 1.5h-2.5A1.75 1.75 0 0110 4.25v-2.5a.75.75 0 01.75-.75zM1 10.75a.75.75 0 01.75-.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a.75.75 0 01-1.5 0v-2.5a.25.25 0 00-.25-.25h-2.5a.75.75 0 01-.75-.75zm9 1c0-.966.784-1.75 1.75-1.75h2.5a.75.75 0 010 1.5h-2.5a.25.25 0 00-.25.25v2.5a.75.75 0 01-1.5 0v-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPin;
impl IconShape for GoPin {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.456.734a1.75 1.75 0 012.826.504l.613 1.327a3.081 3.081 0 002.084 1.707l2.454.584c1.332.317 1.8 1.972.832 2.94L11.06 10l3.72 3.72a.75.75 0 11-1.061 1.06L10 11.06l-2.204 2.205c-.968.968-2.623.5-2.94-.832l-.584-2.454a3.081 3.081 0 00-1.707-2.084l-1.327-.613a1.75 1.75 0 01-.504-2.826L4.456.734zM5.92 1.866a.25.25 0 00-.404-.072L1.794 5.516a.25.25 0 00.072.404l1.328.613A4.582 4.582 0 015.73 9.63l.584 2.454a.25.25 0 00.42.12l5.47-5.47a.25.25 0 00-.12-.42L9.63 5.73a4.581 4.581 0 01-3.098-2.537L5.92 1.866z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitPullRequest;
impl IconShape for GoGitPullRequest {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.177 3.073L9.573.677A.25.25 0 0110 .854v4.792a.25.25 0 01-.427.177L7.177 3.427a.25.25 0 010-.354zM3.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122v5.256a2.251 2.251 0 11-1.5 0V5.372A2.25 2.25 0 011.5 3.25zM11 2.5h-1V4h1a1 1 0 011 1v5.628a2.251 2.251 0 101.5 0V5A2.5 2.5 0 0011 2.5zm1 10.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.75 12a.75.75 0 100 1.5.75.75 0 000-1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiffRemoved;
impl IconShape for GoDiffRemoved {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.75 2.5h10.5a.25.25 0 01.25.25v10.5a.25.25 0 01-.25.25H2.75a.25.25 0 01-.25-.25V2.75a.25.25 0 01.25-.25zM13.25 1H2.75A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1zm-2 7.75a.75.75 0 000-1.5h-6.5a.75.75 0 000 1.5h6.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSignIn;
impl IconShape for GoSignIn {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 2.75C2 1.784 2.784 1 3.75 1h2.5a.75.75 0 010 1.5h-2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h2.5a.75.75 0 010 1.5h-2.5A1.75 1.75 0 012 13.25V2.75zm6.56 4.5l1.97-1.97a.75.75 0 10-1.06-1.06L6.22 7.47a.75.75 0 000 1.06l3.25 3.25a.75.75 0 101.06-1.06L8.56 8.75h5.69a.75.75 0 000-1.5H8.56z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCreditCard;
impl IconShape for GoCreditCard {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.75 9a.75.75 0 000 1.5h1.5a.75.75 0 000-1.5h-1.5z",
            }
            path {
                d: "M0 3.75C0 2.784.784 2 1.75 2h12.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 14H1.75A1.75 1.75 0 010 12.25v-8.5zm14.5 0V5h-13V3.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25zm0 2.75h-13v5.75c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V6.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoServer;
impl IconShape for GoServer {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 1A1.75 1.75 0 000 2.75v4c0 .372.116.717.314 1a1.742 1.742 0 00-.314 1v4c0 .966.784 1.75 1.75 1.75h12.5A1.75 1.75 0 0016 12.75v-4c0-.372-.116-.717-.314-1 .198-.283.314-.628.314-1v-4A1.75 1.75 0 0014.25 1H1.75zm0 7.5a.25.25 0 00-.25.25v4c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-4a.25.25 0 00-.25-.25H1.75zM1.5 2.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v4a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25v-4zm5.5 2A.75.75 0 017.75 4h4.5a.75.75 0 010 1.5h-4.5A.75.75 0 017 4.75zM7.75 10a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5zM3 4.75A.75.75 0 013.75 4h.5a.75.75 0 010 1.5h-.5A.75.75 0 013 4.75zM3.75 10a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoAlert;
impl IconShape for GoAlert {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPaperAirplane;
impl IconShape for GoPaperAirplane {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.592 2.712L2.38 7.25h4.87a.75.75 0 110 1.5H2.38l-.788 4.538L13.929 8 1.592 2.712zM.989 8L.064 2.68a1.341 1.341 0 011.85-1.462l13.402 5.744a1.13 1.13 0 010 2.076L1.913 14.782a1.341 1.341 0 01-1.85-1.463L.99 8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodescanCheckmark;
impl IconShape for GoCodescanCheckmark {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.28 6.28a.75.75 0 10-1.06-1.06L6.25 8.19l-.97-.97a.75.75 0 00-1.06 1.06l1.5 1.5a.75.75 0 001.06 0l3.5-3.5z",
            }
            path {
                d: "M7.5 15a7.469 7.469 0 004.746-1.693l2.474 2.473a.75.75 0 101.06-1.06l-2.473-2.474A7.5 7.5 0 107.5 15zm0-13.5a6 6 0 104.094 10.386.75.75 0 01.293-.292A6 6 0 007.5 1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitCommit;
impl IconShape for GoGitCommit {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.5 7.75a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0zm1.43.75a4.002 4.002 0 01-7.86 0H.75a.75.75 0 110-1.5h3.32a4.001 4.001 0 017.86 0h3.32a.75.75 0 110 1.5h-3.32z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedHeart;
impl IconShape for GoFeedHeart {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm2.33-11.5c-1.22 0-1.83.5-2.323 1.136C7.513 5 6.903 4.5 5.682 4.5c-1.028 0-2.169.784-2.169 2.5 0 1.499 1.493 3.433 3.246 4.517.52.321.89.479 1.248.484.357-.005.728-.163 1.247-.484C11.007 10.433 12.5 8.5 12.5 7c0-1.716-1.14-2.5-2.17-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoEye;
impl IconShape for GoEye {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.679 7.932c.412-.621 1.242-1.75 2.366-2.717C5.175 4.242 6.527 3.5 8 3.5c1.473 0 2.824.742 3.955 1.715 1.124.967 1.954 2.096 2.366 2.717a.119.119 0 010 .136c-.412.621-1.242 1.75-2.366 2.717C10.825 11.758 9.473 12.5 8 12.5c-1.473 0-2.824-.742-3.955-1.715C2.92 9.818 2.09 8.69 1.679 8.068a.119.119 0 010-.136zM8 2c-1.981 0-3.67.992-4.933 2.078C1.797 5.169.88 6.423.43 7.1a1.619 1.619 0 000 1.798c.45.678 1.367 1.932 2.637 3.024C4.329 13.008 6.019 14 8 14c1.981 0 3.67-.992 4.933-2.078 1.27-1.091 2.187-2.345 2.637-3.023a1.619 1.619 0 000-1.798c-.45-.678-1.367-1.932-2.637-3.023C11.671 2.992 9.981 2 8 2zm0 8a2 2 0 100-4 2 2 0 000 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoInfo;
impl IconShape for GoInfo {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm6.5-.25A.75.75 0 017.25 7h1a.75.75 0 01.75.75v2.75h.25a.75.75 0 010 1.5h-2a.75.75 0 010-1.5h.25v-2h-.25a.75.75 0 01-.75-.75zM8 6a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPaintbrush;
impl IconShape for GoPaintbrush {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.134 1.535C9.722 2.562 8.16 4.057 6.889 5.312 5.8 6.387 5.041 7.401 4.575 8.294a3.745 3.745 0 00-3.227 1.054c-.43.431-.69 1.066-.86 1.657a11.982 11.982 0 00-.358 1.914A21.263 21.263 0 000 15.203v.054l.75-.007-.007.75h.054a14.404 14.404 0 00.654-.012 21.243 21.243 0 001.63-.118c.62-.07 1.3-.18 1.914-.357.592-.17 1.226-.43 1.657-.861a3.745 3.745 0 001.055-3.217c.908-.461 1.942-1.216 3.04-2.3 1.279-1.262 2.764-2.825 3.775-4.249.501-.706.923-1.428 1.125-2.096.2-.659.235-1.469-.368-2.07-.606-.607-1.42-.55-2.069-.34-.66.213-1.376.646-2.076 1.155zm-3.95 8.48a3.76 3.76 0 00-1.19-1.192 9.758 9.758 0 011.161-1.607l1.658 1.658a9.853 9.853 0 01-1.63 1.142zM.742 16l.007-.75-.75.008A.75.75 0 00.743 16zM12.016 2.749c-1.224.89-2.605 2.189-3.822 3.384l1.718 1.718c1.21-1.205 2.51-2.597 3.387-3.833.47-.662.78-1.227.912-1.662.134-.444.032-.551.009-.575h-.001V1.78c-.014-.014-.112-.113-.548.027-.432.14-.995.462-1.655.942zM1.62 13.089a19.56 19.56 0 00-.104 1.395 19.55 19.55 0 001.396-.104 10.528 10.528 0 001.668-.309c.526-.151.856-.325 1.011-.48a2.25 2.25 0 00-3.182-3.182c-.155.155-.329.485-.48 1.01a10.515 10.515 0 00-.309 1.67z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSignOut;
impl IconShape for GoSignOut {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 2.75C2 1.784 2.784 1 3.75 1h2.5a.75.75 0 010 1.5h-2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h2.5a.75.75 0 010 1.5h-2.5A1.75 1.75 0 012 13.25V2.75zm10.44 4.5H6.75a.75.75 0 000 1.5h5.69l-1.97 1.97a.75.75 0 101.06 1.06l3.25-3.25a.75.75 0 000-1.06l-3.25-3.25a.75.75 0 10-1.06 1.06l1.97 1.97z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTriangleUp;
impl IconShape for GoTriangleUp {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.427 9.573l3.396-3.396a.25.25 0 01.354 0l3.396 3.396a.25.25 0 01-.177.427H4.604a.25.25 0 01-.177-.427z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFoldDown;
impl IconShape for GoFoldDown {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.177 14.323l2.896-2.896a.25.25 0 00-.177-.427H8.75V7.764a.75.75 0 10-1.5 0V11H5.104a.25.25 0 00-.177.427l2.896 2.896a.25.25 0 00.354 0zM2.25 5a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM6 4.25a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5a.75.75 0 01.75.75zM8.25 5a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM12 4.25a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5a.75.75 0 01.75.75zm2.25.75a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShare;
impl IconShape for GoShare {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.823.177L4.927 3.073a.25.25 0 00.177.427H7.25v5.75a.75.75 0 001.5 0V3.5h2.146a.25.25 0 00.177-.427L8.177.177a.25.25 0 00-.354 0zM3.75 6.5a.25.25 0 00-.25.25v6.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25v-6.5a.25.25 0 00-.25-.25h-1a.75.75 0 010-1.5h1c.966 0 1.75.784 1.75 1.75v6.5A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25v-6.5C2 5.784 2.784 5 3.75 5h1a.75.75 0 110 1.5h-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDeviceMobile;
impl IconShape for GoDeviceMobile {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.75 0A1.75 1.75 0 002 1.75v12.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 14.25V1.75A1.75 1.75 0 0012.25 0h-8.5zM3.5 1.75a.25.25 0 01.25-.25h8.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25V1.75zM8 13a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCircleSlash;
impl IconShape for GoCircleSlash {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 8a6.5 6.5 0 0110.535-5.096l-9.131 9.131A6.472 6.472 0 011.5 8zm2.465 5.096a6.5 6.5 0 009.131-9.131l-9.131 9.131zM8 0a8 8 0 100 16A8 8 0 008 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCommentDiscussion;
impl IconShape for GoCommentDiscussion {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 2.75a.25.25 0 01.25-.25h8.5a.25.25 0 01.25.25v5.5a.25.25 0 01-.25.25h-3.5a.75.75 0 00-.53.22L3.5 11.44V9.25a.75.75 0 00-.75-.75h-1a.25.25 0 01-.25-.25v-5.5zM1.75 1A1.75 1.75 0 000 2.75v5.5C0 9.216.784 10 1.75 10H2v1.543a1.457 1.457 0 002.487 1.03L7.061 10h3.189A1.75 1.75 0 0012 8.25v-5.5A1.75 1.75 0 0010.25 1h-8.5zM14.5 4.75a.25.25 0 00-.25-.25h-.5a.75.75 0 110-1.5h.5c.966 0 1.75.784 1.75 1.75v5.5A1.75 1.75 0 0114.25 12H14v1.543a1.457 1.457 0 01-2.487 1.03L9.22 12.28a.75.75 0 111.06-1.06l2.22 2.22v-2.19a.75.75 0 01.75-.75h1a.25.25 0 00.25-.25v-5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTable;
impl IconShape for GoTable {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v3.585a.746.746 0 010 .83v8.085A1.75 1.75 0 0114.25 16H6.309a.748.748 0 01-1.118 0H1.75A1.75 1.75 0 010 14.25V6.165a.746.746 0 010-.83V1.75zM1.5 6.5v7.75c0 .138.112.25.25.25H5v-8H1.5zM5 5H1.5V1.75a.25.25 0 01.25-.25H5V5zm1.5 1.5v8h7.75a.25.25 0 00.25-.25V6.5h-8zm8-1.5h-8V1.5h7.75a.25.25 0 01.25.25V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArchive;
impl IconShape for GoArchive {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 2.5a.25.25 0 00-.25.25v1.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-1.5a.25.25 0 00-.25-.25H1.75zM0 2.75C0 1.784.784 1 1.75 1h12.5c.966 0 1.75.784 1.75 1.75v1.5A1.75 1.75 0 0114.25 6H1.75A1.75 1.75 0 010 4.25v-1.5zM1.75 7a.75.75 0 01.75.75v5.5c0 .138.112.25.25.25h10.5a.25.25 0 00.25-.25v-5.5a.75.75 0 111.5 0v5.5A1.75 1.75 0 0113.25 15H2.75A1.75 1.75 0 011 13.25v-5.5A.75.75 0 011.75 7zm4.5 1a.75.75 0 000 1.5h3.5a.75.75 0 100-1.5h-3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGraph;
impl IconShape for GoGraph {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 1.75a.75.75 0 00-1.5 0v12.5c0 .414.336.75.75.75h14.5a.75.75 0 000-1.5H1.5V1.75zm14.28 2.53a.75.75 0 00-1.06-1.06L10 7.94 7.53 5.47a.75.75 0 00-1.06 0L3.22 8.72a.75.75 0 001.06 1.06L7 7.06l2.47 2.47a.75.75 0 001.06 0l5.25-5.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSidebarExpand;
impl IconShape for GoSidebarExpand {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.177 7.823l2.396-2.396A.25.25 0 017 5.604v4.792a.25.25 0 01-.427.177L4.177 8.177a.25.25 0 010-.354z",
                fill_rule: "evenodd",
            }
            path {
                d: "M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v12.5A1.75 1.75 0 0114.25 16H1.75A1.75 1.75 0 010 14.25V1.75zm1.75-.25a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25H9.5v-13H1.75zm12.5 13H11v-13h3.25a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiffIgnored;
impl IconShape for GoDiffIgnored {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.75 2.5h10.5a.25.25 0 01.25.25v10.5a.25.25 0 01-.25.25H2.75a.25.25 0 01-.25-.25V2.75a.25.25 0 01.25-.25zM13.25 1H2.75A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1zm-1.97 4.78a.75.75 0 00-1.06-1.06l-5.5 5.5a.75.75 0 101.06 1.06l5.5-5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIssueDraft;
impl IconShape for GoIssueDraft {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.749.097a8.054 8.054 0 012.502 0 .75.75 0 11-.233 1.482 6.554 6.554 0 00-2.036 0A.75.75 0 016.749.097zM4.345 1.693A.75.75 0 014.18 2.74a6.542 6.542 0 00-1.44 1.44.75.75 0 01-1.212-.883 8.042 8.042 0 011.769-1.77.75.75 0 011.048.166zm7.31 0a.75.75 0 011.048-.165 8.04 8.04 0 011.77 1.769.75.75 0 11-1.214.883 6.542 6.542 0 00-1.439-1.44.75.75 0 01-.165-1.047zM.955 6.125a.75.75 0 01.624.857 6.554 6.554 0 000 2.036.75.75 0 01-1.482.233 8.054 8.054 0 010-2.502.75.75 0 01.858-.624zm14.09 0a.75.75 0 01.858.624 8.057 8.057 0 010 2.502.75.75 0 01-1.482-.233 6.55 6.55 0 000-2.036.75.75 0 01.624-.857zm-13.352 5.53a.75.75 0 011.048.165 6.542 6.542 0 001.439 1.44.75.75 0 01-.883 1.212 8.04 8.04 0 01-1.77-1.769.75.75 0 01.166-1.048zm12.614 0a.75.75 0 01.165 1.048 8.038 8.038 0 01-1.769 1.77.75.75 0 11-.883-1.214 6.543 6.543 0 001.44-1.439.75.75 0 011.047-.165zm-8.182 3.39a.75.75 0 01.857-.624 6.55 6.55 0 002.036 0 .75.75 0 01.233 1.482 8.057 8.057 0 01-2.502 0 .75.75 0 01-.624-.858z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSkip;
impl IconShape for GoSkip {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zm3.28 5.78a.75.75 0 00-1.06-1.06l-5.5 5.5a.75.75 0 101.06 1.06l5.5-5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGrabber;
impl IconShape for GoGrabber {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 13a1 1 0 100-2 1 1 0 000 2zm-4 0a1 1 0 100-2 1 1 0 000 2zm1-5a1 1 0 11-2 0 1 1 0 012 0zm3 1a1 1 0 100-2 1 1 0 000 2zm1-5a1 1 0 11-2 0 1 1 0 012 0zM6 5a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCheckCircle;
impl IconShape for GoCheckCircle {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM0 8a8 8 0 1116 0A8 8 0 010 8zm11.78-1.72a.75.75 0 00-1.06-1.06L6.75 9.19 5.28 7.72a.75.75 0 00-1.06 1.06l2 2a.75.75 0 001.06 0l4.5-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileDirectoryFill;
impl IconShape for GoFileDirectoryFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3H7.5a.25.25 0 01-.2-.1l-.9-1.2C6.07 1.26 5.55 1 5 1H1.75z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodeSquare;
impl IconShape for GoCodeSquare {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 1.5a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V1.75a.25.25 0 00-.25-.25H1.75zM0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v12.5A1.75 1.75 0 0114.25 16H1.75A1.75 1.75 0 010 14.25V1.75zm9.22 3.72a.75.75 0 000 1.06L10.69 8 9.22 9.47a.75.75 0 101.06 1.06l2-2a.75.75 0 000-1.06l-2-2a.75.75 0 00-1.06 0zM6.78 6.53a.75.75 0 00-1.06-1.06l-2 2a.75.75 0 000 1.06l2 2a.75.75 0 101.06-1.06L5.31 8l1.47-1.47z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileAdded;
impl IconShape for GoFileAdded {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.75 1.5a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 00.25-.25V4.664a.25.25 0 00-.073-.177l-2.914-2.914a.25.25 0 00-.177-.073H3.75zM2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0113.25 16h-9.5A1.75 1.75 0 012 14.25V1.75zm6.23 3.508a.75.75 0 01.755.745l.01 1.497h1.497a.75.75 0 010 1.5H9v1.507a.75.75 0 01-1.5 0V9.005l-1.502.01a.75.75 0 11-.01-1.5l1.507-.01-.01-1.492a.75.75 0 01.745-.755z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDeviceCameraVideo;
impl IconShape for GoDeviceCameraVideo {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 3.75a.75.75 0 00-1.136-.643L11 5.425V4.75A1.75 1.75 0 009.25 3h-7.5A1.75 1.75 0 000 4.75v6.5C0 12.216.784 13 1.75 13h7.5A1.75 1.75 0 0011 11.25v-.675l3.864 2.318A.75.75 0 0016 12.25v-8.5zm-5 5.075l3.5 2.1v-5.85l-3.5 2.1v1.65zM9.5 6.75v-2a.25.25 0 00-.25-.25h-7.5a.25.25 0 00-.25.25v6.5c0 .138.112.25.25.25h7.5a.25.25 0 00.25-.25v-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMoon;
impl IconShape for GoMoon {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.598 1.591a.75.75 0 01.785-.175 7 7 0 11-8.967 8.967.75.75 0 01.961-.96 5.5 5.5 0 007.046-7.046.75.75 0 01.175-.786zm1.616 1.945a7 7 0 01-7.678 7.678 5.5 5.5 0 107.678-7.678z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLaw;
impl IconShape for GoLaw {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.75.75a.75.75 0 00-1.5 0V2h-.984c-.305 0-.604.08-.869.23l-1.288.737A.25.25 0 013.984 3H1.75a.75.75 0 000 1.5h.428L.066 9.192a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.514 3.514 0 00.686.45A4.492 4.492 0 003 11c.88 0 1.556-.22 2.023-.454a3.515 3.515 0 00.686-.45l.045-.04.016-.015.006-.006.002-.002.001-.002L5.25 9.5l.53.53a.75.75 0 00.154-.838L3.822 4.5h.162c.305 0 .604-.08.869-.23l1.289-.737a.25.25 0 01.124-.033h.984V13h-2.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-2.5V3.5h.984a.25.25 0 01.124.033l1.29.736c.264.152.563.231.868.231h.162l-2.112 4.692a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.517 3.517 0 00.686.45A4.492 4.492 0 0013 11c.88 0 1.556-.22 2.023-.454a3.512 3.512 0 00.686-.45l.045-.04.01-.01.006-.005.006-.006.002-.002.001-.002-.529-.531.53.53a.75.75 0 00.154-.838L13.823 4.5h.427a.75.75 0 000-1.5h-2.234a.25.25 0 01-.124-.033l-1.29-.736A1.75 1.75 0 009.735 2H8.75V.75zM1.695 9.227c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L3 6.327l-1.305 2.9zm10 0c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L13 6.327l-1.305 2.9z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTelescope;
impl IconShape for GoTelescope {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.184 1.143a1.75 1.75 0 00-2.502-.57L.912 7.916a1.75 1.75 0 00-.53 2.32l.447.775a1.75 1.75 0 002.275.702l11.745-5.656a1.75 1.75 0 00.757-2.451l-1.422-2.464zm-1.657.669a.25.25 0 01.358.081l1.422 2.464a.25.25 0 01-.108.35l-2.016.97-1.505-2.605 1.85-1.26zM9.436 3.92l1.391 2.41-5.42 2.61-.942-1.63 4.97-3.39zM3.222 8.157l-1.466 1a.25.25 0 00-.075.33l.447.775a.25.25 0 00.325.1l1.598-.769-.83-1.436zm6.253 2.306a.75.75 0 00-.944-.252l-1.809.87a.75.75 0 00-.293.253L4.38 14.326a.75.75 0 101.238.848l1.881-2.75v2.826a.75.75 0 001.5 0v-2.826l1.881 2.75a.75.75 0 001.238-.848l-2.644-3.863z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDotFill;
impl IconShape for GoDotFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 4a4 4 0 100 8 4 4 0 000-8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoQuestion;
impl IconShape for GoQuestion {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm9 3a1 1 0 11-2 0 1 1 0 012 0zM6.92 6.085c.081-.16.19-.299.34-.398.145-.097.371-.187.74-.187.28 0 .553.087.738.225A.613.613 0 019 6.25c0 .177-.04.264-.077.318a.956.956 0 01-.277.245c-.076.051-.158.1-.258.161l-.007.004a7.728 7.728 0 00-.313.195 2.416 2.416 0 00-.692.661.75.75 0 001.248.832.956.956 0 01.276-.245 6.3 6.3 0 01.26-.16l.006-.004c.093-.057.204-.123.313-.195.222-.149.487-.355.692-.662.214-.32.329-.702.329-1.15 0-.76-.36-1.348-.863-1.725A2.76 2.76 0 008 4c-.631 0-1.155.16-1.572.438-.413.276-.68.638-.849.977a.75.75 0 101.342.67z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSync;
impl IconShape for GoSync {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 2.5a5.487 5.487 0 00-4.131 1.869l1.204 1.204A.25.25 0 014.896 6H1.25A.25.25 0 011 5.75V2.104a.25.25 0 01.427-.177l1.38 1.38A7.001 7.001 0 0114.95 7.16a.75.75 0 11-1.49.178A5.501 5.501 0 008 2.5zM1.705 8.005a.75.75 0 01.834.656 5.501 5.501 0 009.592 2.97l-1.204-1.204a.25.25 0 01.177-.427h3.646a.25.25 0 01.25.25v3.646a.25.25 0 01-.427.177l-1.38-1.38A7.001 7.001 0 011.05 8.84a.75.75 0 01.656-.834z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoWebhook;
impl IconShape for GoWebhook {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.5 4.25a2.25 2.25 0 014.5 0 .75.75 0 001.5 0 3.75 3.75 0 10-6.14 2.889l-2.272 4.258a.75.75 0 001.324.706L7 7.25a.75.75 0 00-.309-1.015A2.25 2.25 0 015.5 4.25z",
            }
            path {
                d: "M7.364 3.607a.75.75 0 011.03.257l2.608 4.349a3.75 3.75 0 11-.628 6.785.75.75 0 01.752-1.299 2.25 2.25 0 10-.033-3.88.75.75 0 01-1.03-.256L7.107 4.636a.75.75 0 01.257-1.03z",
            }
            path {
                d: "M2.9 8.776A.75.75 0 012.625 9.8 2.25 2.25 0 106 11.75a.75.75 0 01.75-.751h5.5a.75.75 0 010 1.5H7.425a3.751 3.751 0 11-5.55-3.998.75.75 0 011.024.274z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPlug;
impl IconShape for GoPlug {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.276 3.09a.25.25 0 01.192-.09h.782a.25.25 0 01.25.25v8.5a.25.25 0 01-.25.25h-.782a.25.25 0 01-.192-.09l-.95-1.14a.75.75 0 00-.483-.264l-3.124-.39a.25.25 0 01-.219-.249V5.133a.25.25 0 01.219-.248l3.124-.39a.75.75 0 00.483-.265l.95-1.14zM4 8v1.867a1.75 1.75 0 001.533 1.737l2.83.354.761.912c.332.4.825.63 1.344.63h.782A1.75 1.75 0 0013 11.75V11h2.25a.75.75 0 000-1.5H13v-4h2.25a.75.75 0 000-1.5H13v-.75a1.75 1.75 0 00-1.75-1.75h-.782c-.519 0-1.012.23-1.344.63l-.76.913-2.831.353A1.75 1.75 0 004 5.133V6.5H2.5A2.5 2.5 0 000 9v5.25a.75.75 0 001.5 0V9a1 1 0 011-1H4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoUnmute;
impl IconShape for GoUnmute {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.563 2.069A.75.75 0 018 2.75v10.5a.75.75 0 01-1.238.57L3.472 11H1.75A1.75 1.75 0 010 9.25v-2.5C0 5.784.784 5 1.75 5h1.723l3.289-2.82a.75.75 0 01.801-.111zM6.5 4.38L4.238 6.319a.75.75 0 01-.488.181h-2a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h2a.75.75 0 01.488.18L6.5 11.62V4.38zm6.096-2.038a.75.75 0 011.06 0 8 8 0 010 11.314.75.75 0 01-1.06-1.06 6.5 6.5 0 000-9.193.75.75 0 010-1.06v-.001zm-1.06 2.121a.75.75 0 10-1.061 1.061 3.5 3.5 0 010 4.95.75.75 0 101.06 1.06 5 5 0 000-7.07l.001-.001z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDuplicate;
impl IconShape for GoDuplicate {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.5 3a.75.75 0 01.75.75v1h1a.75.75 0 010 1.5h-1v1a.75.75 0 01-1.5 0v-1h-1a.75.75 0 010-1.5h1v-1A.75.75 0 0110.5 3z",
            }
            path {
                d: "M6.75 0A1.75 1.75 0 005 1.75v7.5c0 .966.784 1.75 1.75 1.75h7.5A1.75 1.75 0 0016 9.25v-7.5A1.75 1.75 0 0014.25 0h-7.5zM6.5 1.75a.25.25 0 01.25-.25h7.5a.25.25 0 01.25.25v7.5a.25.25 0 01-.25.25h-7.5a.25.25 0 01-.25-.25v-7.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M1.75 5A1.75 1.75 0 000 6.75v7.5C0 15.216.784 16 1.75 16h7.5A1.75 1.75 0 0011 14.25v-1.5a.75.75 0 00-1.5 0v1.5a.25.25 0 01-.25.25h-7.5a.25.25 0 01-.25-.25v-7.5a.25.25 0 01.25-.25h1.5a.75.75 0 000-1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPlus;
impl IconShape for GoPlus {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.75 2a.75.75 0 01.75.75V7h4.25a.75.75 0 110 1.5H8.5v4.25a.75.75 0 11-1.5 0V8.5H2.75a.75.75 0 010-1.5H7V2.75A.75.75 0 017.75 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoProject;
impl IconShape for GoProject {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoReport;
impl IconShape for GoReport {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 1.5a.25.25 0 00-.25.25v9.5c0 .138.112.25.25.25h2a.75.75 0 01.75.75v2.19l2.72-2.72a.75.75 0 01.53-.22h6.5a.25.25 0 00.25-.25v-9.5a.25.25 0 00-.25-.25H1.75zM0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v9.5A1.75 1.75 0 0114.25 13H8.06l-2.573 2.573A1.457 1.457 0 013 14.543V13H1.75A1.75 1.75 0 010 11.25v-9.5zM9 9a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPeople;
impl IconShape for GoPeople {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.5 3.5a2 2 0 100 4 2 2 0 000-4zM2 5.5a3.5 3.5 0 115.898 2.549 5.507 5.507 0 013.034 4.084.75.75 0 11-1.482.235 4.001 4.001 0 00-7.9 0 .75.75 0 01-1.482-.236A5.507 5.507 0 013.102 8.05 3.49 3.49 0 012 5.5zM11 4a.75.75 0 100 1.5 1.5 1.5 0 01.666 2.844.75.75 0 00-.416.672v.352a.75.75 0 00.574.73c1.2.289 2.162 1.2 2.522 2.372a.75.75 0 101.434-.44 5.01 5.01 0 00-2.56-3.012A3 3 0 0011 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPencil;
impl IconShape for GoPencil {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.013 1.427a1.75 1.75 0 012.474 0l1.086 1.086a1.75 1.75 0 010 2.474l-8.61 8.61c-.21.21-.47.364-.756.445l-3.251.93a.75.75 0 01-.927-.928l.929-3.25a1.75 1.75 0 01.445-.758l8.61-8.61zm1.414 1.06a.25.25 0 00-.354 0L10.811 3.75l1.439 1.44 1.263-1.263a.25.25 0 000-.354l-1.086-1.086zM11.189 6.25L9.75 4.81l-6.286 6.287a.25.25 0 00-.064.108l-.558 1.953 1.953-.558a.249.249 0 00.108-.064l6.286-6.286z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSquare;
impl IconShape for GoSquare {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 5.75C4 4.784 4.784 4 5.75 4h4.5c.966 0 1.75.784 1.75 1.75v4.5A1.75 1.75 0 0110.25 12h-4.5A1.75 1.75 0 014 10.25v-4.5zm1.75-.25a.25.25 0 00-.25.25v4.5c0 .138.112.25.25.25h4.5a.25.25 0 00.25-.25v-4.5a.25.25 0 00-.25-.25h-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoWorkflow;
impl IconShape for GoWorkflow {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 1.75C0 .784.784 0 1.75 0h3.5C6.216 0 7 .784 7 1.75v3.5A1.75 1.75 0 015.25 7H4v4a1 1 0 001 1h4v-1.25C9 9.784 9.784 9 10.75 9h3.5c.966 0 1.75.784 1.75 1.75v3.5A1.75 1.75 0 0114.25 16h-3.5A1.75 1.75 0 019 14.25v-.75H5A2.5 2.5 0 012.5 11V7h-.75A1.75 1.75 0 010 5.25v-3.5zm1.75-.25a.25.25 0 00-.25.25v3.5c0 .138.112.25.25.25h3.5a.25.25 0 00.25-.25v-3.5a.25.25 0 00-.25-.25h-3.5zm9 9a.25.25 0 00-.25.25v3.5c0 .138.112.25.25.25h3.5a.25.25 0 00.25-.25v-3.5a.25.25 0 00-.25-.25h-3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDesktopDownload;
impl IconShape for GoDesktopDownload {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.927 5.427l2.896 2.896a.25.25 0 00.354 0l2.896-2.896A.25.25 0 0010.896 5H8.75V.75a.75.75 0 10-1.5 0V5H5.104a.25.25 0 00-.177.427z",
            }
            path {
                d: "M1.573 2.573a.25.25 0 00-.073.177v7.5a.25.25 0 00.25.25h12.5a.25.25 0 00.25-.25v-7.5a.25.25 0 00-.25-.25h-3a.75.75 0 110-1.5h3A1.75 1.75 0 0116 2.75v7.5A1.75 1.75 0 0114.25 12h-3.727c.099 1.041.52 1.872 1.292 2.757A.75.75 0 0111.25 16h-6.5a.75.75 0 01-.565-1.243c.772-.885 1.192-1.716 1.292-2.757H1.75A1.75 1.75 0 010 10.25v-7.5A1.75 1.75 0 011.75 1h3a.75.75 0 010 1.5h-3a.25.25 0 00-.177.073zM6.982 12a5.72 5.72 0 01-.765 2.5h3.566a5.72 5.72 0 01-.765-2.5H6.982z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFilter;
impl IconShape for GoFilter {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M.75 3a.75.75 0 000 1.5h14.5a.75.75 0 000-1.5H.75zM3 7.75A.75.75 0 013.75 7h8.5a.75.75 0 010 1.5h-8.5A.75.75 0 013 7.75zm3 4a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5a.75.75 0 01-.75-.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBookmarkSlash;
impl IconShape for GoBookmarkSlash {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.19 1.143a.75.75 0 10-.88 1.214L3 4.305v9.945a.75.75 0 001.206.596L8 11.944l3.794 2.902A.75.75 0 0013 14.25v-2.703l1.81 1.31a.75.75 0 10.88-1.214l-2.994-2.168a1.09 1.09 0 00-.014-.01L4.196 3.32a.712.712 0 00-.014-.01L1.19 1.143zM4.5 5.39v7.341l3.044-2.328a.75.75 0 01.912 0l3.044 2.328V10.46l-7-5.07zM5.865 1a.75.75 0 000 1.5h5.385a.25.25 0 01.25.25v3.624a.75.75 0 001.5 0V2.75A1.75 1.75 0 0011.25 1H5.865z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileZip;
impl IconShape for GoFileZip {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.5 1.75a.25.25 0 01.25-.25h3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h2.086a.25.25 0 01.177.073l2.914 2.914a.25.25 0 01.073.177v8.586a.25.25 0 01-.25.25h-.5a.75.75 0 000 1.5h.5A1.75 1.75 0 0014 13.25V4.664c0-.464-.184-.909-.513-1.237L10.573.513A1.75 1.75 0 009.336 0H3.75A1.75 1.75 0 002 1.75v11.5c0 .649.353 1.214.874 1.515a.75.75 0 10.752-1.298.25.25 0 01-.126-.217V1.75zM8.75 3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM6 5.25a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5A.75.75 0 016 5.25zm2 1.5A.75.75 0 018.75 6h.5a.75.75 0 010 1.5h-.5A.75.75 0 018 6.75zm-1.25.75a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM8 9.75A.75.75 0 018.75 9h.5a.75.75 0 010 1.5h-.5A.75.75 0 018 9.75zm-.75.75a1.75 1.75 0 00-1.75 1.75v3c0 .414.336.75.75.75h2.5a.75.75 0 00.75-.75v-3a1.75 1.75 0 00-1.75-1.75h-.5zM7 12.25a.25.25 0 01.25-.25h.5a.25.25 0 01.25.25v2.25H7v-2.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSliders;
impl IconShape for GoSliders {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 2.75a.75.75 0 01-.75.75h-4a.75.75 0 010-1.5h4a.75.75 0 01.75.75zm-8.5.75v1.25a.75.75 0 001.5 0v-4a.75.75 0 00-1.5 0V2H1.75a.75.75 0 000 1.5H6.5zm1.25 5.25a.75.75 0 000-1.5h-6a.75.75 0 000 1.5h6zM15 8a.75.75 0 01-.75.75H11.5V10a.75.75 0 11-1.5 0V6a.75.75 0 011.5 0v1.25h2.75A.75.75 0 0115 8zm-9 5.25v-2a.75.75 0 00-1.5 0v1.25H1.75a.75.75 0 000 1.5H4.5v1.25a.75.75 0 001.5 0v-2zm9 0a.75.75 0 01-.75.75h-6a.75.75 0 010-1.5h6a.75.75 0 01.75.75z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCopilotError;
impl IconShape for GoCopilotError {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.86 1.77c.05.053.097.107.14.164.043-.057.09-.111.14-.164.681-.731 1.737-.9 2.943-.765 1.23.136 2.145.527 2.724 1.26.566.716.693 1.614.693 2.485 0 .572-.053 1.147-.254 1.655l.168.838.066.033A2.75 2.75 0 0116 9.736V11c0 .24-.086.438-.156.567a1.755 1.755 0 01-.075.125L13 9.688V7.824l-.023-.115c-.49.21-1.075.291-1.727.291-.22 0-.43-.012-.633-.036L6.824 5.22c.082-.233.143-.503.182-.813.117-.936-.038-1.396-.242-1.614-.193-.207-.637-.414-1.681-.298-.707.079-1.144.243-1.424.434l-1.251-.905c.58-.579 1.422-.899 2.51-1.02 1.205-.133 2.26.035 2.943.766zm1.376 1.023c.193-.207.637-.414 1.681-.298 1.02.114 1.48.404 1.713.7.247.313.37.79.37 1.555 0 .792-.129 1.17-.308 1.37-.162.181-.52.38-1.442.38-.854 0-1.339-.236-1.638-.54-.315-.323-.527-.827-.618-1.553-.117-.936.038-1.396.242-1.614zM.865 2.759A.75.75 0 00.31 4.107l1.193.864c.013.498.076.992.251 1.434l-.167.838-.067.033A2.75 2.75 0 000 9.736V11c0 .24.086.438.156.567.075.137.169.261.259.366.18.21.404.413.605.58a10.368 10.368 0 00.792.597l.015.01.006.004.028.018.098.065a12.06 12.06 0 001.654.859C4.704 14.527 6.244 15 8 15c1.756 0 3.296-.472 4.387-.935.395-.167.734-.335 1.008-.482l1.415 1.024a.75.75 0 001.063-1.025.753.753 0 01-.188-.1L.865 2.76zM4.75 8c.297 0 .579-.022.844-.066l6.427 4.654c-.07.032-.144.064-.22.097-.972.412-2.307.815-3.801.815-1.494 0-2.83-.403-3.8-.815a10.594 10.594 0 01-1.2-.6v-4.26l.023-.116c.49.21 1.075.291 1.727.291z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoComment;
impl IconShape for GoComment {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.75 2.5a.25.25 0 00-.25.25v7.5c0 .138.112.25.25.25h2a.75.75 0 01.75.75v2.19l2.72-2.72a.75.75 0 01.53-.22h4.5a.25.25 0 00.25-.25v-7.5a.25.25 0 00-.25-.25H2.75zM1 2.75C1 1.784 1.784 1 2.75 1h10.5c.966 0 1.75.784 1.75 1.75v7.5A1.75 1.75 0 0113.25 12H9.06l-2.573 2.573A1.457 1.457 0 014 13.543V12H2.75A1.75 1.75 0 011 10.25v-7.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoNoEntry;
impl IconShape for GoNoEntry {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.25 7.25a.75.75 0 000 1.5h7.5a.75.75 0 000-1.5h-7.5z",
            }
            path {
                d: "M16 8A8 8 0 110 8a8 8 0 0116 0zm-1.5 0a6.5 6.5 0 11-13 0 6.5 6.5 0 0113 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowRight;
impl IconShape for GoArrowRight {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.22 2.97a.75.75 0 011.06 0l4.25 4.25a.75.75 0 010 1.06l-4.25 4.25a.75.75 0 01-1.06-1.06l2.97-2.97H3.75a.75.75 0 010-1.5h7.44L8.22 4.03a.75.75 0 010-1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiffAdded;
impl IconShape for GoDiffAdded {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.25 2.5H2.75a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h10.5a.25.25 0 00.25-.25V2.75a.25.25 0 00-.25-.25zM2.75 1h10.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0113.25 15H2.75A1.75 1.75 0 011 13.25V2.75C1 1.784 1.784 1 2.75 1zM8 4a.75.75 0 01.75.75v2.5h2.5a.75.75 0 010 1.5h-2.5v2.5a.75.75 0 01-1.5 0v-2.5h-2.5a.75.75 0 010-1.5h2.5v-2.5A.75.75 0 018 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoClock;
impl IconShape for GoClock {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zm.5 4.75a.75.75 0 00-1.5 0v3.5a.75.75 0 00.471.696l2.5 1a.75.75 0 00.557-1.392L8.5 7.742V4.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCopilotWarning;
impl IconShape for GoCopilotWarning {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.86 1.77c.05.053.097.107.14.164.043-.057.09-.111.14-.164.681-.731 1.737-.9 2.943-.765 1.23.136 2.145.527 2.724 1.26.566.716.693 1.614.693 2.485 0 .463-.035.929-.155 1.359a5.967 5.967 0 00-1.398-.616c.034-.195.053-.439.053-.743 0-.766-.123-1.242-.37-1.555-.233-.296-.693-.586-1.713-.7-1.044-.116-1.488.091-1.681.298-.204.218-.359.678-.242 1.614.06.479.172.86.332 1.158a6.014 6.014 0 00-2.92 2.144C5.926 7.904 5.372 8 4.75 8c-.652 0-1.237-.082-1.727-.291L3 7.824v4.261c.02.013.043.025.065.038a10.84 10.84 0 002.495 1.035c.21.629.522 1.21.916 1.726a11.91 11.91 0 01-2.863-.819 12.06 12.06 0 01-1.296-.641 8.815 8.815 0 01-.456-.281l-.028-.02-.006-.003-.015-.01a7.077 7.077 0 01-.235-.166c-.15-.108-.352-.26-.557-.43a5.19 5.19 0 01-.605-.58 2.167 2.167 0 01-.259-.367A1.19 1.19 0 010 11V9.736a2.75 2.75 0 011.52-2.46l.067-.033.167-.838C1.553 5.897 1.5 5.322 1.5 4.75c0-.87.127-1.77.693-2.485.579-.733 1.494-1.124 2.724-1.26 1.206-.134 2.262.034 2.944.765zM6.765 2.793c-.193-.207-.637-.414-1.681-.298-1.02.114-1.48.404-1.713.7-.247.313-.37.79-.37 1.555 0 .792.129 1.17.308 1.37.162.181.52.38 1.442.38.854 0 1.339-.236 1.638-.54.315-.323.527-.827.618-1.553.117-.936-.038-1.396-.242-1.614z",
                fill_rule: "evenodd",
            }
            path {
                d: "M8.498 14.81a4.5 4.5 0 105.504-7.121 4.5 4.5 0 00-5.504 7.122zM10.5 8.75a.75.75 0 011.5 0V11a.75.75 0 01-1.5 0V8.75zm.75 5.75a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBeaker;
impl IconShape for GoBeaker {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 5.782V2.5h-.25a.75.75 0 010-1.5h6.5a.75.75 0 010 1.5H11v3.282l3.666 5.76C15.619 13.04 14.543 15 12.767 15H3.233c-1.776 0-2.852-1.96-1.899-3.458L5 5.782zM9.5 2.5h-3V6a.75.75 0 01-.117.403L4.73 9h6.54L9.617 6.403A.75.75 0 019.5 6V2.5zm-6.9 9.847L3.775 10.5h8.45l1.175 1.847a.75.75 0 01-.633 1.153H3.233a.75.75 0 01-.633-1.153z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedDiscussion;
impl IconShape for GoFeedDiscussion {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zM4 5a1 1 0 011-1h6a1 1 0 011 1v5a1 1 0 01-1 1H8.707l-1.853 1.854A.5.5 0 016 12.5V11H5a1 1 0 01-1-1V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPlay;
impl IconShape for GoPlay {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zM6.379 5.227A.25.25 0 006 5.442v5.117a.25.25 0 00.379.214l4.264-2.559a.25.25 0 000-.428L6.379 5.227z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIdBadge;
impl IconShape for GoIdBadge {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 7.5a.5.5 0 01.5-.5h2a.5.5 0 01.5.5v3a.5.5 0 01-.5.5h-2a.5.5 0 01-.5-.5v-3zm10 .25a.75.75 0 01-.75.75h-4.5a.75.75 0 010-1.5h4.5a.75.75 0 01.75.75zM10.25 11a.75.75 0 000-1.5h-2.5a.75.75 0 000 1.5h2.5z",
            }
            path {
                d: "M7.25 0A1.75 1.75 0 005.5 1.75V3H1.75A1.75 1.75 0 000 4.75v8.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3H10.5V1.75A1.75 1.75 0 008.75 0h-1.5zm3.232 4.5A1.75 1.75 0 018.75 6h-1.5a1.75 1.75 0 01-1.732-1.5H1.75a.25.25 0 00-.25.25v8.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-8.5a.25.25 0 00-.25-.25h-3.768zM7 1.75a.25.25 0 01.25-.25h1.5a.25.25 0 01.25.25v2.5a.25.25 0 01-.25.25h-1.5A.25.25 0 017 4.25v-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMention;
impl IconShape for GoMention {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.75 2.37a6.5 6.5 0 006.5 11.26.75.75 0 01.75 1.298 8 8 0 113.994-7.273.754.754 0 01.006.095v1.5a2.75 2.75 0 01-5.072 1.475A4 4 0 1112 8v1.25a1.25 1.25 0 002.5 0V7.867a6.5 6.5 0 00-9.75-5.496V2.37zM10.5 8a2.5 2.5 0 10-5 0 2.5 2.5 0 005 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoScreenFull;
impl IconShape for GoScreenFull {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.75 2.5a.25.25 0 00-.25.25v2.5a.75.75 0 01-1.5 0v-2.5C1 1.784 1.784 1 2.75 1h2.5a.75.75 0 010 1.5h-2.5zM10 1.75a.75.75 0 01.75-.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a.75.75 0 01-1.5 0v-2.5a.25.25 0 00-.25-.25h-2.5a.75.75 0 01-.75-.75zM1.75 10a.75.75 0 01.75.75v2.5c0 .138.112.25.25.25h2.5a.75.75 0 010 1.5h-2.5A1.75 1.75 0 011 13.25v-2.5a.75.75 0 01.75-.75zm12.5 0a.75.75 0 01.75.75v2.5A1.75 1.75 0 0113.25 15h-2.5a.75.75 0 010-1.5h2.5a.25.25 0 00.25-.25v-2.5a.75.75 0 01.75-.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBook;
impl IconShape for GoBook {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 1.75A.75.75 0 01.75 1h4.253c1.227 0 2.317.59 3 1.501A3.744 3.744 0 0111.006 1h4.245a.75.75 0 01.75.75v10.5a.75.75 0 01-.75.75h-4.507a2.25 2.25 0 00-1.591.659l-.622.621a.75.75 0 01-1.06 0l-.622-.621A2.25 2.25 0 005.258 13H.75a.75.75 0 01-.75-.75V1.75zm8.755 3a2.25 2.25 0 012.25-2.25H14.5v9h-3.757c-.71 0-1.4.201-1.992.572l.004-7.322zm-1.504 7.324l.004-5.073-.002-2.253A2.25 2.25 0 005.003 2.5H1.5v9h3.757a3.75 3.75 0 011.994.574z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStopwatch;
impl IconShape for GoStopwatch {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.75.75A.75.75 0 016.5 0h3a.75.75 0 010 1.5h-.75v1l-.001.041a6.718 6.718 0 013.464 1.435l.007-.006.75-.75a.75.75 0 111.06 1.06l-.75.75-.006.007a6.75 6.75 0 11-10.548 0L2.72 5.03l-.75-.75a.75.75 0 011.06-1.06l.75.75.007.006A6.718 6.718 0 017.25 2.541a.756.756 0 010-.041v-1H6.5a.75.75 0 01-.75-.75zM8 14.5A5.25 5.25 0 108 4a5.25 5.25 0 000 10.5zm.389-6.7l1.33-1.33a.75.75 0 111.061 1.06L9.45 8.861A1.502 1.502 0 018 10.75a1.5 1.5 0 11.389-2.95z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCheckCircleFill;
impl IconShape for GoCheckCircleFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm3.78-9.72a.75.75 0 00-1.06-1.06L6.75 9.19 5.28 7.72a.75.75 0 00-1.06 1.06l2 2a.75.75 0 001.06 0l4.5-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIssueReopened;
impl IconShape for GoIssueReopened {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.029 2.217a6.5 6.5 0 019.437 5.11.75.75 0 101.492-.154 8 8 0 00-14.315-4.03L.427 1.927A.25.25 0 000 2.104V5.75A.25.25 0 00.25 6h3.646a.25.25 0 00.177-.427L2.715 4.215a6.491 6.491 0 012.314-1.998zM1.262 8.169a.75.75 0 00-1.22.658 8.001 8.001 0 0014.315 4.03l1.216 1.216a.25.25 0 00.427-.177V10.25a.25.25 0 00-.25-.25h-3.646a.25.25 0 00-.177.427l1.358 1.358a6.501 6.501 0 01-11.751-3.11.75.75 0 00-.272-.506z",
            }
            path {
                d: "M9.06 9.06a1.5 1.5 0 11-2.12-2.12 1.5 1.5 0 012.12 2.12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBlocked;
impl IconShape for GoBlocked {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.467.22a.75.75 0 01.53-.22h6.006a.75.75 0 01.53.22l4.247 4.247c.141.14.22.331.22.53v6.006a.75.75 0 01-.22.53l-4.247 4.247a.75.75 0 01-.53.22H4.997a.75.75 0 01-.53-.22L.22 11.533a.75.75 0 01-.22-.53V4.997a.75.75 0 01.22-.53L4.467.22zm.84 1.28L1.5 5.308v5.384L5.308 14.5h5.384l3.808-3.808V5.308L10.692 1.5H5.308zM4 7.75A.75.75 0 014.75 7h6.5a.75.75 0 010 1.5h-6.5A.75.75 0 014 7.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHash;
impl IconShape for GoHash {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.368 1.01a.75.75 0 01.623.859L6.57 4.5h3.98l.46-2.868a.75.75 0 011.48.237L12.07 4.5h2.18a.75.75 0 010 1.5h-2.42l-.64 4h2.56a.75.75 0 010 1.5h-2.8l-.46 2.869a.75.75 0 01-1.48-.237l.42-2.632H5.45l-.46 2.869a.75.75 0 01-1.48-.237l.42-2.632H1.75a.75.75 0 010-1.5h2.42l.64-4H2.25a.75.75 0 010-1.5h2.8l.46-2.868a.75.75 0 01.858-.622zM9.67 10l.64-4H6.33l-.64 4h3.98z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedMerged;
impl IconShape for GoFeedMerged {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm.25-11.25a1.75 1.75 0 01-1.207 1.664A2 2 0 009 8h.571a1.75 1.75 0 110 1H9a2.99 2.99 0 01-2-.764v1.336a1.75 1.75 0 11-1 0V6.428A1.75 1.75 0 118.25 4.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHeartFill;
impl IconShape for GoHeartFill {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.655 14.916L8 14.25l.345.666a.752.752 0 01-.69 0zm0 0L8 14.25l.345.666.002-.001.006-.003.018-.01a7.643 7.643 0 00.31-.17 22.08 22.08 0 003.433-2.414C13.956 10.731 16 8.35 16 5.5 16 2.836 13.914 1 11.75 1 10.203 1 8.847 1.802 8 3.02 7.153 1.802 5.797 1 4.25 1 2.086 1 0 2.836 0 5.5c0 2.85 2.045 5.231 3.885 6.818a22.075 22.075 0 003.744 2.584l.018.01.006.003h.002z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSquirrel;
impl IconShape for GoSquirrel {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.499.75a.75.75 0 011.5 0v.996C5.9 2.903 6.793 3.65 7.662 4.376l.24.202c-.036-.694.055-1.422.426-2.163C9.1.873 10.794-.045 12.622.26 14.408.558 16 1.94 16 4.25c0 1.278-.954 2.575-2.44 2.734l.146.508.065.22c.203.701.412 1.455.476 2.226.142 1.707-.4 3.03-1.487 3.898C11.714 14.671 10.27 15 8.75 15h-6a.75.75 0 010-1.5h1.376a4.489 4.489 0 01-.563-1.191 3.833 3.833 0 01-.05-2.063 4.636 4.636 0 01-2.025-.293.75.75 0 11.525-1.406c1.357.507 2.376-.006 2.698-.318l.009-.01a.748.748 0 011.06 0 .75.75 0 01-.012 1.074c-.912.92-.992 1.835-.768 2.586.221.74.745 1.337 1.196 1.621H8.75c1.343 0 2.398-.296 3.074-.836.635-.507 1.036-1.31.928-2.602-.05-.603-.216-1.224-.422-1.93l-.064-.221c-.12-.407-.246-.84-.353-1.29a2.404 2.404 0 01-.507-.441 3.063 3.063 0 01-.633-1.248.75.75 0 011.455-.364c.046.185.144.436.31.627.146.168.353.305.712.305.738 0 1.25-.615 1.25-1.25 0-1.47-.95-2.315-2.123-2.51-1.172-.196-2.227.387-2.706 1.345-.46.92-.27 1.774.019 3.062l.042.19a.753.753 0 01.01.05c.348.443.666.949.94 1.553a.75.75 0 11-1.365.62c-.553-1.217-1.32-1.94-2.3-2.768a85.08 85.08 0 00-.317-.265c-.814-.68-1.75-1.462-2.692-2.619a3.74 3.74 0 00-1.023.88c-.406.495-.663 1.036-.722 1.508.116.122.306.21.591.239.388.038.797-.06 1.032-.19a.75.75 0 01.728 1.31c-.515.287-1.23.439-1.906.373-.682-.067-1.473-.38-1.879-1.193L.75 5.677V5.5c0-.984.48-1.94 1.077-2.664.46-.559 1.05-1.055 1.673-1.353V.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoImage;
impl IconShape for GoImage {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h.94a.76.76 0 01.03-.03l6.077-6.078a1.75 1.75 0 012.412-.06L14.5 10.31V2.75a.25.25 0 00-.25-.25H1.75zm12.5 11H4.81l5.048-5.047a.25.25 0 01.344-.009l4.298 3.889v.917a.25.25 0 01-.25.25zm1.75-.25V2.75A1.75 1.75 0 0014.25 1H1.75A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25zM5.5 6a.5.5 0 11-1 0 .5.5 0 011 0zM7 6a2 2 0 11-4 0 2 2 0 014 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCode;
impl IconShape for GoCode {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.72 3.22a.75.75 0 011.06 1.06L2.06 8l3.72 3.72a.75.75 0 11-1.06 1.06L.47 8.53a.75.75 0 010-1.06l4.25-4.25zm6.56 0a.75.75 0 10-1.06 1.06L13.94 8l-3.72 3.72a.75.75 0 101.06 1.06l4.25-4.25a.75.75 0 000-1.06l-4.25-4.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTrash;
impl IconShape for GoTrash {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.5 1.75a.25.25 0 01.25-.25h2.5a.25.25 0 01.25.25V3h-3V1.75zm4.5 0V3h2.25a.75.75 0 010 1.5H2.75a.75.75 0 010-1.5H5V1.75C5 .784 5.784 0 6.75 0h2.5C10.216 0 11 .784 11 1.75zM4.496 6.675a.75.75 0 10-1.492.15l.66 6.6A1.75 1.75 0 005.405 15h5.19c.9 0 1.652-.681 1.741-1.576l.66-6.6a.75.75 0 00-1.492-.149l-.66 6.6a.25.25 0 01-.249.225h-5.19a.25.25 0 01-.249-.225l-.66-6.6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoClone;
impl IconShape for GoRepoClone {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 0H9v7c0 .55.45 1 1 1h1v1h1V8h3c.55 0 1-.45 1-1V1c0-.55-.45-1-1-1zm-4 7h-1V6h1v1zm4 0h-3V6h3v1zm0-2h-4V1h4v4zM4 5H3V4h1v1zm0-2H3V2h1v1zM2 1h6V0H1C.45 0 0 .45 0 1v12c0 .55.45 1 1 1h2v2l1.5-1.5L6 16v-2h5c.55 0 1-.45 1-1v-3H2V1zm9 10v2H6v-1H3v1H1v-2h10zM3 8h1v1H3V8zm1-1H3V6h1v1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRows;
impl IconShape for GoRows {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 2.75A1.75 1.75 0 0014.25 1H1.75A1.75 1.75 0 000 2.75v2.5A1.75 1.75 0 001.75 7h12.5A1.75 1.75 0 0016 5.25v-2.5zm-1.75-.25a.25.25 0 01.25.25v2.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25v-2.5a.25.25 0 01.25-.25h12.5zM16 10.75A1.75 1.75 0 0014.25 9H1.75A1.75 1.75 0 000 10.75v2.5A1.75 1.75 0 001.75 15h12.5A1.75 1.75 0 0016 13.25v-2.5zm-1.75-.25a.25.25 0 01.25.25v2.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25v-2.5a.25.25 0 01.25-.25h12.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCalendar;
impl IconShape for GoCalendar {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.75 0a.75.75 0 01.75.75V2h5V.75a.75.75 0 011.5 0V2h1.25c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0113.25 16H2.75A1.75 1.75 0 011 14.25V3.75C1 2.784 1.784 2 2.75 2H4V.75A.75.75 0 014.75 0zm0 3.5h8.5a.25.25 0 01.25.25V6h-11V3.75a.25.25 0 01.25-.25h2zm-2.25 4v6.75c0 .138.112.25.25.25h10.5a.25.25 0 00.25-.25V7.5h-11z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoKebabHorizontal;
impl IconShape for GoKebabHorizontal {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zM1.5 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zm13 0a1.5 1.5 0 100-3 1.5 1.5 0 000 3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPersonAdd;
impl IconShape for GoPersonAdd {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.25 0a.75.75 0 01.75.75V2h1.25a.75.75 0 010 1.5H14v1.25a.75.75 0 01-1.5 0V3.5h-1.25a.75.75 0 010-1.5h1.25V.75a.75.75 0 01.75-.75zM5.5 4a2 2 0 100 4 2 2 0 000-4zm2.4 4.548a3.5 3.5 0 10-4.799 0 5.527 5.527 0 00-3.1 4.66.75.75 0 101.498.085A4.01 4.01 0 015.5 9.5a4.01 4.01 0 014.001 3.793.75.75 0 101.498-.086 5.527 5.527 0 00-3.1-4.659z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShieldCheck;
impl IconShape for GoShieldCheck {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.533.133a1.75 1.75 0 00-1.066 0l-5.25 1.68A1.75 1.75 0 001 3.48V7c0 1.566.32 3.182 1.303 4.682.983 1.498 2.585 2.813 5.032 3.855a1.7 1.7 0 001.33 0c2.447-1.042 4.049-2.357 5.032-3.855C14.68 10.182 15 8.566 15 7V3.48a1.75 1.75 0 00-1.217-1.667L8.533.133zm-.61 1.429a.25.25 0 01.153 0l5.25 1.68a.25.25 0 01.174.238V7c0 1.358-.275 2.666-1.057 3.86-.784 1.194-2.121 2.34-4.366 3.297a.2.2 0 01-.154 0c-2.245-.956-3.582-2.104-4.366-3.298C2.775 9.666 2.5 8.36 2.5 7V3.48a.25.25 0 01.174-.237l5.25-1.68zM11.28 6.28a.75.75 0 00-1.06-1.06L7.25 8.19l-.97-.97a.75.75 0 10-1.06 1.06l1.5 1.5a.75.75 0 001.06 0l3.5-3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStack;
impl IconShape for GoStack {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.122.392a1.75 1.75 0 011.756 0l5.003 2.902c.83.481.83 1.68 0 2.162L8.878 8.358a1.75 1.75 0 01-1.756 0L2.119 5.456a1.25 1.25 0 010-2.162L7.122.392zM8.125 1.69a.25.25 0 00-.25 0l-4.63 2.685 4.63 2.685a.25.25 0 00.25 0l4.63-2.685-4.63-2.685zM1.601 7.789a.75.75 0 011.025-.273l5.249 3.044a.25.25 0 00.25 0l5.249-3.044a.75.75 0 01.752 1.298l-5.248 3.044a1.75 1.75 0 01-1.756 0L1.874 8.814A.75.75 0 011.6 7.789zm0 3.5a.75.75 0 011.025-.273l5.249 3.044a.25.25 0 00.25 0l5.249-3.044a.75.75 0 01.752 1.298l-5.248 3.044a1.75 1.75 0 01-1.756 0l-5.248-3.044a.75.75 0 01-.273-1.025z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMail;
impl IconShape for GoMail {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.75 2A1.75 1.75 0 000 3.75v.736a.75.75 0 000 .027v7.737C0 13.216.784 14 1.75 14h12.5A1.75 1.75 0 0016 12.25v-8.5A1.75 1.75 0 0014.25 2H1.75zM14.5 4.07v-.32a.25.25 0 00-.25-.25H1.75a.25.25 0 00-.25.25v.32L8 7.88l6.5-3.81zm-13 1.74v6.441c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V5.809L8.38 9.397a.75.75 0 01-.76 0L1.5 5.809z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoThreeBars;
impl IconShape for GoThreeBars {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 2.75A.75.75 0 011.75 2h12.5a.75.75 0 110 1.5H1.75A.75.75 0 011 2.75zm0 5A.75.75 0 011.75 7h12.5a.75.75 0 110 1.5H1.75A.75.75 0 011 7.75zM1.75 12a.75.75 0 100 1.5h12.5a.75.75 0 100-1.5H1.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTasklist;
impl IconShape for GoTasklist {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.5 2.75a.25.25 0 01.25-.25h10.5a.25.25 0 01.25.25v10.5a.25.25 0 01-.25.25H2.75a.25.25 0 01-.25-.25V2.75zM2.75 1A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1H2.75zm9.03 5.28a.75.75 0 00-1.06-1.06L6.75 9.19 5.28 7.72a.75.75 0 00-1.06 1.06l2 2a.75.75 0 001.06 0l4.5-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSingleSelect;
impl IconShape for GoSingleSelect {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.06 7.356l2.795 2.833c.08.081.21.081.29 0l2.794-2.833c.13-.131.038-.356-.145-.356H5.206c-.183 0-.275.225-.145.356z",
            }
            path {
                d: "M1 2.75C1 1.784 1.784 1 2.75 1h10.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0113.25 15H2.75A1.75 1.75 0 011 13.25V2.75zm1.75-.25a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h10.5a.25.25 0 00.25-.25V2.75a.25.25 0 00-.25-.25H2.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoZap;
impl IconShape for GoZap {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.561 1.5a.016.016 0 00-.01.004L3.286 8.571A.25.25 0 003.462 9H6.75a.75.75 0 01.694 1.034l-1.713 4.188 6.982-6.793A.25.25 0 0012.538 7H9.25a.75.75 0 01-.683-1.06l2.008-4.418.003-.006a.02.02 0 00-.004-.009.02.02 0 00-.006-.006L10.56 1.5zM9.504.43a1.516 1.516 0 012.437 1.713L10.415 5.5h2.123c1.57 0 2.346 1.909 1.22 3.004l-7.34 7.142a1.25 1.25 0 01-.871.354h-.302a1.25 1.25 0 01-1.157-1.723L5.633 10.5H3.462c-1.57 0-2.346-1.909-1.22-3.004L9.503.429z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLock;
impl IconShape for GoLock {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4v2h-.25A1.75 1.75 0 002 7.75v5.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-5.5A1.75 1.75 0 0012.25 6H12V4a4 4 0 10-8 0zm6.5 2V4a2.5 2.5 0 00-5 0v2h5zM12 7.5h.25a.25.25 0 01.25.25v5.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-5.5a.25.25 0 01.25-.25H12z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCircle;
impl IconShape for GoCircle {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMarkGithub;
impl IconShape for GoMarkGithub {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoEllipsis;
impl IconShape for GoEllipsis {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 5.75C0 4.784.784 4 1.75 4h12.5c.966 0 1.75.784 1.75 1.75v4.5A1.75 1.75 0 0114.25 12H1.75A1.75 1.75 0 010 10.25v-4.5zM4 7a1 1 0 100 2 1 1 0 000-2zm3 1a1 1 0 112 0 1 1 0 01-2 0zm5-1a1 1 0 100 2 1 1 0 000-2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoTemplate;
impl IconShape for GoRepoTemplate {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 .75A.75.75 0 016.75 0h2.5a.75.75 0 010 1.5h-2.5A.75.75 0 016 .75zm5 0a.75.75 0 01.75-.75h1.5a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0V1.5h-.75A.75.75 0 0111 .75zM4.992.662a.75.75 0 01-.636.848c-.436.063-.783.41-.846.846a.75.75 0 01-1.485-.212A2.501 2.501 0 014.144.025a.75.75 0 01.848.637zM2.75 4a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5A.75.75 0 012.75 4zm10.5 0a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5a.75.75 0 01.75-.75zM2.75 8a.75.75 0 01.75.75v.268A1.72 1.72 0 013.75 9h.5a.75.75 0 010 1.5h-.5a.25.25 0 00-.25.25v.75c0 .28.114.532.3.714a.75.75 0 01-1.05 1.072A2.495 2.495 0 012 11.5V8.75A.75.75 0 012.75 8zm10.5 0a.75.75 0 01.75.75v4.5a.75.75 0 01-.75.75h-2.5a.75.75 0 010-1.5h1.75v-2h-.75a.75.75 0 010-1.5h.75v-.25a.75.75 0 01.75-.75zM6 9.75A.75.75 0 016.75 9h2.5a.75.75 0 010 1.5h-2.5A.75.75 0 016 9.75zm-1 2.5v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHome;
impl IconShape for GoHome {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.156 1.835a.25.25 0 00-.312 0l-5.25 4.2a.25.25 0 00-.094.196v7.019c0 .138.112.25.25.25H5.5V8.25a.75.75 0 01.75-.75h3.5a.75.75 0 01.75.75v5.25h2.75a.25.25 0 00.25-.25V6.23a.25.25 0 00-.094-.195l-5.25-4.2zM6.906.664a1.75 1.75 0 012.187 0l5.25 4.2c.415.332.657.835.657 1.367v7.019A1.75 1.75 0 0113.25 15h-3.5a.75.75 0 01-.75-.75V9H7v5.25a.75.75 0 01-.75.75h-3.5A1.75 1.75 0 011 13.25V6.23c0-.531.242-1.034.657-1.366l5.25-4.2h-.001z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoNumber;
impl IconShape for GoNumber {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.604.089A.75.75 0 016 .75v4.77h.711a.75.75 0 110 1.5H3.759a.75.75 0 110-1.5H4.5V2.15l-.334.223a.75.75 0 01-.832-1.248l1.5-1a.75.75 0 01.77-.037zM9 4.75A.75.75 0 019.75 4h4a.75.75 0 01.53 1.28l-1.89 1.892c.312.076.604.18.867.319.742.391 1.244 1.063 1.244 2.005 0 .653-.231 1.208-.629 1.627-.386.408-.894.653-1.408.777-1.01.243-2.225.063-3.124-.527a.75.75 0 01.822-1.254c.534.35 1.32.474 1.951.322.306-.073.53-.201.67-.349.129-.136.218-.32.218-.596 0-.308-.123-.509-.444-.678-.373-.197-.98-.318-1.806-.318a.75.75 0 01-.53-1.28l1.72-1.72H9.75A.75.75 0 019 4.75zm-3.587 5.763c-.35-.05-.77.113-.983.572a.75.75 0 11-1.36-.632c.508-1.094 1.589-1.565 2.558-1.425 1 .145 1.872.945 1.872 2.222 0 1.433-1.088 2.192-1.79 2.681-.308.216-.571.397-.772.573H7a.75.75 0 010 1.5H3.75a.75.75 0 01-.75-.75c0-.69.3-1.211.67-1.61.348-.372.8-.676 1.15-.92.8-.56 1.18-.904 1.18-1.474 0-.473-.267-.69-.587-.737z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShareAndroid;
impl IconShape for GoShareAndroid {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5 3a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM15 3a3 3 0 01-5.175 2.066l-3.92 2.179a3.005 3.005 0 010 1.51l3.92 2.179a3 3 0 11-.73 1.31l-3.92-2.178a3 3 0 110-4.133l3.92-2.178A3 3 0 1115 3zm-1.5 10a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zm-9-5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSidebarCollapse;
impl IconShape for GoSidebarCollapse {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.823 7.823L4.427 5.427A.25.25 0 004 5.604v4.792c0 .223.27.335.427.177l2.396-2.396a.25.25 0 000-.354z",
                fill_rule: "evenodd",
            }
            path {
                d: "M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25H9.5v13H1.75a.25.25 0 01-.25-.25V1.75zM11 14.5v-13h3.25a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H11z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoVersions;
impl IconShape for GoVersions {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.75 14A1.75 1.75 0 016 12.25v-8.5C6 2.784 6.784 2 7.75 2h6.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 14h-6.5zm-.25-1.75c0 .138.112.25.25.25h6.5a.25.25 0 00.25-.25v-8.5a.25.25 0 00-.25-.25h-6.5a.25.25 0 00-.25.25v8.5zM4.9 3.508a.75.75 0 01-.274 1.025.25.25 0 00-.126.217v6.5a.25.25 0 00.126.217.75.75 0 01-.752 1.298A1.75 1.75 0 013 11.25v-6.5c0-.649.353-1.214.874-1.516a.75.75 0 011.025.274zM1.625 5.533a.75.75 0 10-.752-1.299A1.75 1.75 0 000 5.75v4.5c0 .649.353 1.214.874 1.515a.75.75 0 10.752-1.298.25.25 0 01-.126-.217v-4.5a.25.25 0 01.126-.217z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileCode;
impl IconShape for GoFileCode {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 1.75C4 .784 4.784 0 5.75 0h5.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v8.586A1.75 1.75 0 0114.25 15h-9a.75.75 0 010-1.5h9a.25.25 0 00.25-.25V6h-2.75A1.75 1.75 0 0110 4.25V1.5H5.75a.25.25 0 00-.25.25v2.5a.75.75 0 01-1.5 0v-2.5zm7.5-.188V4.25c0 .138.112.25.25.25h2.688a.252.252 0 00-.011-.013l-2.914-2.914a.272.272 0 00-.013-.011zM5.72 6.72a.75.75 0 000 1.06l1.47 1.47-1.47 1.47a.75.75 0 101.06 1.06l2-2a.75.75 0 000-1.06l-2-2a.75.75 0 00-1.06 0zM3.28 7.78a.75.75 0 00-1.06-1.06l-2 2a.75.75 0 000 1.06l2 2a.75.75 0 001.06-1.06L1.81 9.25l1.47-1.47z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSearch;
impl IconShape for GoSearch {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoListOrdered;
impl IconShape for GoListOrdered {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.003 2.5a.5.5 0 00-.723-.447l-1.003.5a.5.5 0 00.446.895l.28-.14V6H.5a.5.5 0 000 1h2.006a.5.5 0 100-1h-.503V2.5zM5 3.25a.75.75 0 01.75-.75h8.5a.75.75 0 010 1.5h-8.5A.75.75 0 015 3.25zm0 5a.75.75 0 01.75-.75h8.5a.75.75 0 010 1.5h-8.5A.75.75 0 015 8.25zm0 5a.75.75 0 01.75-.75h8.5a.75.75 0 010 1.5h-8.5a.75.75 0 01-.75-.75zM.924 10.32l.003-.004a.851.851 0 01.144-.153A.66.66 0 011.5 10c.195 0 .306.068.374.146a.57.57 0 01.128.376c0 .453-.269.682-.8 1.078l-.035.025C.692 11.98 0 12.495 0 13.5a.5.5 0 00.5.5h2.003a.5.5 0 000-1H1.146c.132-.197.351-.372.654-.597l.047-.035c.47-.35 1.156-.858 1.156-1.845 0-.365-.118-.744-.377-1.038-.268-.303-.658-.484-1.126-.484-.48 0-.84.202-1.068.392a1.858 1.858 0 00-.348.384l-.007.011-.002.004-.001.002-.001.001a.5.5 0 00.851.525zM.5 10.055l-.427-.26.427.26z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoDeleted;
impl IconShape for GoRepoDeleted {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 2.5A2.5 2.5 0 013.5 0h8.75a.75.75 0 01.75.75v7.5a.75.75 0 01-1.5 0V1.5h-8a1 1 0 00-1 1v6.708A2.492 2.492 0 013.5 9h4.75a.75.75 0 010 1.5H3.5a1 1 0 100 2h4.75a.75.75 0 010 1.5H3.5A2.5 2.5 0 011 11.5v-9z",
            }
            path {
                d: "M11.28 10.22a.75.75 0 10-1.06 1.06L11.94 13l-1.72 1.72a.75.75 0 101.06 1.06L13 14.06l1.72 1.72a.75.75 0 101.06-1.06L14.06 13l1.72-1.72a.75.75 0 10-1.06-1.06L13 11.94l-1.72-1.72z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPulse;
impl IconShape for GoPulse {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 2a.75.75 0 01.696.471L10 10.731l1.304-3.26A.75.75 0 0112 7h3.25a.75.75 0 010 1.5h-2.742l-1.812 4.528a.75.75 0 01-1.392 0L6 4.77 4.696 8.03A.75.75 0 014 8.5H.75a.75.75 0 010-1.5h2.742l1.812-4.529A.75.75 0 016 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodespaces;
impl IconShape for GoCodespaces {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 1.75C2 .784 2.784 0 3.75 0h8.5C13.216 0 14 .784 14 1.75v5a1.75 1.75 0 01-1.75 1.75h-8.5A1.75 1.75 0 012 6.75v-5zm1.75-.25a.25.25 0 00-.25.25v5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25v-5a.25.25 0 00-.25-.25h-8.5zM0 11.25c0-.966.784-1.75 1.75-1.75h12.5c.966 0 1.75.784 1.75 1.75v3A1.75 1.75 0 0114.25 16H1.75A1.75 1.75 0 010 14.25v-3zM1.75 11a.25.25 0 00-.25.25v3c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-3a.25.25 0 00-.25-.25H1.75z",
                fill_rule: "evenodd",
            }
            path {
                d: "M3 12.75a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5a.75.75 0 01-.75-.75zm4 0a.75.75 0 01.75-.75h4.5a.75.75 0 010 1.5h-4.5a.75.75 0 01-.75-.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoChevronRight;
impl IconShape for GoChevronRight {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.22 3.22a.75.75 0 011.06 0l4.25 4.25a.75.75 0 010 1.06l-4.25 4.25a.75.75 0 01-1.06-1.06L9.94 8 6.22 4.28a.75.75 0 010-1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoReply;
impl IconShape for GoReply {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.78 1.97a.75.75 0 010 1.06L3.81 6h6.44A4.75 4.75 0 0115 10.75v2.5a.75.75 0 01-1.5 0v-2.5a3.25 3.25 0 00-3.25-3.25H3.81l2.97 2.97a.75.75 0 11-1.06 1.06L1.47 7.28a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHourglass;
impl IconShape for GoHourglass {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.75 1a.75.75 0 000 1.5h.75v1.25a4.75 4.75 0 001.9 3.8l.333.25c.134.1.134.3 0 .4l-.333.25a4.75 4.75 0 00-1.9 3.8v1.25h-.75a.75.75 0 000 1.5h10.5a.75.75 0 000-1.5h-.75v-1.25a4.75 4.75 0 00-1.9-3.8l-.333-.25a.25.25 0 010-.4l.333-.25a4.75 4.75 0 001.9-3.8V2.5h.75a.75.75 0 000-1.5H2.75zM11 2.5H5v1.25a3.25 3.25 0 001.3 2.6l.333.25c.934.7.934 2.1 0 2.8l-.333.25a3.25 3.25 0 00-1.3 2.6v1.25h6v-1.25a3.25 3.25 0 00-1.3-2.6l-.333-.25a1.75 1.75 0 010-2.8l.333-.25a3.25 3.25 0 001.3-2.6V2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIssueClosed;
impl IconShape for GoIssueClosed {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.28 6.78a.75.75 0 00-1.06-1.06L7.25 8.69 5.78 7.22a.75.75 0 00-1.06 1.06l2 2a.75.75 0 001.06 0l3.5-3.5z",
            }
            path {
                d: "M16 8A8 8 0 110 8a8 8 0 0116 0zm-1.5 0a6.5 6.5 0 11-13 0 6.5 6.5 0 0113 0z",
                fill_rule: "evenodd",
            }
        }
    }
}
