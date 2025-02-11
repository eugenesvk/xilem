//! An illustration of the various options for Xilem's Label View (based on Masonry Label Widget)
#![cfg_attr(not(debug_assertions),allow(non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]

// TODOs:
  // add rust code generating each element in a tooltip
  // add the same code in a context menu as a "copy" command
  // add URL support for doc links
  // add non-desktop platforms
  // add to a CI build so you can simply download a binary and use it as a reference
    // have both debug/release builds so that you can use F11/F12 inspectors?
      // is a release build useful for basic examples?

use winit::window::Window;
use masonry::dpi::LogicalSize;
use xilem::view::{MainAxisAlignment,CrossAxisAlignment,FlexExt,FlexParams,};
#[derive(Default)]
struct AppState {}

use winit::error::EventLoopError;
use xilem::view::{flex,sized_box,portal,grid,label,prose, Prose, Label,
  GridExt, Padding,
  Axis,
  label as l,
  };
use xilem::{EventLoop,WidgetView,Xilem,FontWeight,TextAlignment,LineBreaking,palette::css,};
use masonry::core::ArcStr;
use masonry::parley::fontique;
use masonry::peniko::Color;
const label_color:Color = css::ROYAL_BLUE;

fn title_prose(text:impl Into<ArcStr>) -> Prose {
  prose(text).text_size(18.0).alignment(TextAlignment::Justified).brush(css::GREEN)
}
fn txt_prose(text:impl Into<ArcStr>) -> Prose {
  prose(text).text_size(14.0).alignment(TextAlignment::Start).brush(Color::from_rgb8(0x11,0x11,0x11))
}
fn lc(text:impl Into<ArcStr>) -> Label { //colored label
  label(text).brush(label_color)
}
fn app_logic(D:&mut AppState) -> impl WidgetView<AppState> {
  let cb = |D:&mut AppState|{}; // empty callback for empty buttons
  // let lc = Color::from_rgb8(0x77,0x77,0x77); //119 css::LIGHT_GRAY=211 label color
  // let lc = Color::from_rgb8(0x0a,0x0a,0x0a);
  let lC = css::ROYAL_BLUE;
  let tC = css::GREEN; //title
  let mC = Color::from_rgb8(0x11,0x11,0x11); //main text
  let mut i = 1;

  portal(
  flex((
  (txt_prose("Xilem view::Label formats vGit@25-02 #25b12ad (in a ↕-scrollable area)").text_size(18.0),
  if cfg!(debug_assertions) {txt_prose(
   "This is a debug build, so you can use github.com/linebender/xilem/tree/main/masonry#debugging-features:
    • F11 to toggle a rudimentary widget inspector
    • F12 to toggle paint widget layout rectangles")
  } else {txt_prose("This is not a debug build, so F11 widget inspector and F12 widget rectangles tools are not available)\ngithub.com/linebender/xilem/tree/main/masonry#debugging-features")},
  label(format!("Label: Serif Bold 14 {label_color:?}")).text_size(14.0).weight(FontWeight::BOLD) // float bold=700, FontWeight::parse("normal") for css
    .font(fontique::GenericFamily::Serif)
    .alignment(TextAlignment::Start)
    .brush(lC) //impl Into<peniko:Brush> brush sets text color, but gradients and images are also supported Solid(color::AlphaColor<Srgb>) Gradient(Gradient) Image(Image),
    .line_break_mode(LineBreaking::Overflow) //WordWrap Clip Overflow
    ,
  title_prose(format!("§{i} .alignment")),{i+=1;},
  txt_prose("  4 options: ≝Start Middle End Justified\n  https://docs.rs/parley/latest/parley/layout/enum.Alignment.html")
  ),
  // doesn't seem to be different now vs unconstrained
  // (lc("  •flex in a 200×70 box to show impact of constraints ").alignment(TextAlignment::Start),
  // sized_box(
  //   flex((
  //     lc("1/4 alignment Start"    	).alignment(TextAlignment::Start    	),
  //     lc("2/4 alignment Middle"   	).alignment(TextAlignment::Middle   	),
  //     lc("3/4 alignment End"      	).alignment(TextAlignment::End      	),
  //     lc("4/4 alignment Justified"	).alignment(TextAlignment::Justified	),
  //   ))
  //   ).width(200f64).height(70f64).padding(Padding::from(0.))
  //    .background(css::LIGHT_GRAY) // .border(css::RED,0.).rounded(RoundedRectRadii::from_single_radius(0.))
  // ,),
  (l("  •grid in a 200×70 sized_box to make labels same-width (one per row in 4×1 table)").alignment(TextAlignment::Justified).brush(mC),
  sized_box(
    grid((
      lc("1/4 alignment Start"    	).alignment(TextAlignment::Start    	).grid_pos(0,0),
      lc("2/4 alignment Middle"   	).alignment(TextAlignment::Middle   	).grid_pos(0,1),
      lc("3/4 alignment End"      	).alignment(TextAlignment::End      	).grid_pos(0,2),
      lc("4/4 alignment Justified"	).alignment(TextAlignment::Justified	).grid_pos(0,3),
      ),1,4,).spacing(0.0)
    ).width(200f64).height(70f64).padding(Padding::from(0.))
     .background(css::LIGHT_GRAY) //.border(css::RED,0.).rounded(RoundedRectRadii::from_single_radius(0.))
  ,),
  (l("  •unboxed (constrained by root parent's flex in a portal)\n  (Start=Middle: parent Flex container ≝CrossAxisAlignment::Center,\n  so the alignment for a label starts at the center)").alignment(TextAlignment::Justified).brush(mC),
  l("  can be fixed with a custom per-element override .flex(FlexParams::new(1.0,CrossAxisAlignment::Start))").alignment(TextAlignment::Justified).brush(mC),
  lc("1/4 alignment Start"    	).alignment(TextAlignment::Start    	),
  lc("2/4 alignment Middle"   	).alignment(TextAlignment::Middle   	),
  lc("3/4 alignment End"      	).alignment(TextAlignment::End      	),
  lc("4/4 alignment Justified"	).alignment(TextAlignment::Justified	),
  ),
  (l("  •flex in a 500×140 sized_box (🐞? unboxed .flex override removes Portal scrolling)").alignment(TextAlignment::Justified).brush(mC),
  sized_box(flex((
    lc("1/4 alignment Start"                             	).alignment(TextAlignment::Start    	),
    lc("1/4 alignment Start + CrossAxisAlignment::Start "	).alignment(TextAlignment::Start    	).flex(FlexParams::new(1.0,CrossAxisAlignment::Start)),
    lc("2/4 alignment Middle"                            	).alignment(TextAlignment::Middle   	),
    lc("3/4 alignment End"                               	).alignment(TextAlignment::End      	),
    lc("4/4 alignment Justified"                         	).alignment(TextAlignment::Justified	),
    ))
    ).width(500f64).height(140f64).padding(Padding::from(0.))
     .background(css::LIGHT_GRAY) //.border(css::RED,0.).rounded(RoundedRectRadii::from_single_radius(0.))
  ),
  (title_prose(format!("§{i} .line_break_mode")),{i+=1;},
  txt_prose("  3 options: ≝WordWrap Clip Overflow\n  https://docs.rs/masonry/latest/masonry/widget/enum.LineBreaking.html"),
  l("  •grid in a 340×120 box to make labels same-width (one per row in 3×1 table)").alignment(TextAlignment::Justified).brush(mC),
  sized_box(
    grid((
      lc("1/3 WordWrap: break at word boundaries = abcd-efgh-ijkl-mnop-qrst-uvwx-yz" 	).line_break_mode(LineBreaking::WordWrap	).grid_pos(0,0),
      lc("2/3 Clip    : truncate to label's width = abcd-efgh-ijkl-mnop-qrst-uvwx-yz"	).line_break_mode(LineBreaking::Clip    	).grid_pos(0,1),
      lc("3/3 Overflow: overflow the label = abcd-efgh-ijkl-mnop-qrst-uvwx-yz"       	).line_break_mode(LineBreaking::Overflow	).grid_pos(0,2),
      ),1,3,).spacing(0.0)
    ).width(340f64).height(120f64).padding(Padding::from(0.))
     .background(css::LIGHT_GRAY)
  ),

  (title_prose(format!("§{i}a .font")),
  txt_prose(" (some options might be invisible due to missing fonts. 🐞❓font substitution?)"),
  flex((
    (lc("1Times New Roman"	).font("Times New Roman"	),
    lc("2Arial"           	).font("Arial"          	),
    lc("3Cambria"         	).font("Cambria"        	),
    lc("4Cambria Math"    	).font("Cambria Math"   	),
    lc("5Verdana"         	).font("Verdana"        	),
    ),
  )).direction(Axis::Horizontal),
  title_prose(format!("§{i}b .font(fontique::GenericFamily::↓)")),{i+=1;},
  flex((
    lc("Serif"    	).font(fontique::GenericFamily::Serif    	),
    lc("SansSerif"	).font(fontique::GenericFamily::SansSerif	),
    lc("Monospace"	).font(fontique::GenericFamily::Monospace	),
    lc("Cursive"  	).font(fontique::GenericFamily::Cursive  	),
    lc("Fantasy"  	).font(fontique::GenericFamily::Fantasy  	),
  )).direction(Axis::Horizontal),
  flex((
    lc("SystemUi"   	).font(fontique::GenericFamily::SystemUi   	),
    lc("UiSerif"    	).font(fontique::GenericFamily::UiSerif    	),
    lc("UiSansSerif"	).font(fontique::GenericFamily::UiSansSerif	),
    lc("UiMonospace"	).font(fontique::GenericFamily::UiMonospace	),
    lc("UiRounded"  	).font(fontique::GenericFamily::UiRounded  	),
  )).direction(Axis::Horizontal),
  flex((
    lc("Emoji"       	).font(fontique::GenericFamily::Emoji   	),
    lc("Math→"       	)                                       	 ,
    lc("⊂⊃Ψ⌈Δ∇∵ℕ⇑⇑₇∞"	).font(fontique::GenericFamily::Math    	),
    lc("FangSong"    	).font(fontique::GenericFamily::FangSong	),
  )).direction(Axis::Horizontal),
  ),

  (title_prose(format!("§{i} Unsupported Masonry options")),{i+=1;},
  txt_prose("  hinting, disabled color, styles (underline, strikethrough, word/letter spacing, font features etc. https://docs.rs/parley/latest/parley/style/enum.StyleProperty.html)"),
  ),
  ))//flex
   .direction(Axis::Vertical) //.main_axis_alignment(MainAxisAlignment::SpaceBetween).cross_axis_alignment(CrossAxisAlignment::Fill)
  ) //portal
}

fn main() -> Result<(), EventLoopError> {
  let app_state_init = AppState::default();
  let xapp = Xilem::new(app_state_init, app_logic)
    .background_color(css::SEASHELL);

  let win_attr = Window::default_attributes()
    .with_title("Label: Xilem View")
    .with_min_inner_size(LogicalSize::new(800., 600.))
    ;

  xapp.run_windowed_in(EventLoop::with_user_event(), win_attr)?;
  Ok(())
}
