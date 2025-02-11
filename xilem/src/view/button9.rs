// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

pub use masonry::core::{PointerButton, PointerButton as PointerB, WidgetPod};
use masonry::widgets::{self, button9 as widget9};

use crate::core::{ViewPathTracker,
  DynMessage, Mut, View, ViewMarker,
  MessageResult, ViewId, MessageResult as MsgRes,
};
use crate::view::Label;
use crate::{Pod, ViewCtx};

use masonry::kurbo::Insets;

/// A button which calls `callback` when the 🖰1 (normally left) is pressed<br>
/// To use button provide it with a button text or `label` and a closure
/// ```ignore
/// use xilem::view::{button, label};
/// struct State { int: i32 }
/// impl   State { fn increase(&mut self) { self.int += 1 ;} }
/// let   label =       "Button"; // or ↓
/// //let label = label("Button").weight(FontWeight::BOLD);
/// button(label, |state:&mut State| { state.increase();})
/// ```
pub fn button9<State,Action> (label:impl Into<Label>, callback:impl Fn(&mut State) -> Action+Send+'static)
->ButtonL9<impl for<'a> Fn(&'a mut State, PointerB) -> MsgRes<Action>+Send+'static> {
  button9_pad(label, None, callback)
}
/// A button with custom `pad` padding which calls `callback` when 🖰1 (normally left) is pressed
pub fn button9_pad<State,Action>(label:impl Into<Label>, pad:Option<Insets>, callback: impl Fn(&mut State) -> Action+Send+'static)
->ButtonL9<impl for<'a> Fn(&'a mut State, PointerB) -> MsgRes<Action>+Send+'static> {
  ButtonL9::new(label, pad,
    move |state: &mut State, button| match button {
      PointerB::Primary => MsgRes::Action(callback(state)),
      _                 => MsgRes::Nop                    ,},
  )
}
/// A button which calls `callback` when any 🖰 is pressed
pub fn button9_any_pointer<State,Action>(label:impl Into<Label>, callback: impl Fn(&mut State, PointerB) -> Action+Send+'static)
->ButtonL9<impl for<'a> Fn(&'a mut State, PointerB) -> MsgRes<Action>+Send+'static> {
  button9_any_pointer_pad(label, None, callback)
}
/// A button with custom `pad` padding which calls `callback` when any 🖰 is pressed
pub fn button9_any_pointer_pad<State,Action>(label:impl Into<Label>, pad:Option<Insets>, callback: impl Fn(&mut State, PointerB) -> Action+Send+'static)
->ButtonL9<impl for<'a> Fn(&'a mut State, PointerB) -> MsgRes<Action>+Send+'static> {
  ButtonL9::new(label, pad,
    move |state: &mut State, button| MsgRes::Action(callback(state, button)),
  )
}

use crate::masonry::button9::{LPos, LabelOpt, Pad9};

/// The [`View`] created by [`button`] from up to label(s) in one of [`LPos`] position with custom padding and a callback.
#[must_use = "View values do nothing unless provided to Xilem."]
pub struct ButtonL9<F> {
  label: Label9,
  opt  : LabelOpt,
  callback: F,
}
/// Label for ButtonL9
pub struct Label9 {
  p1:Label, p2:Label, p3:Label, // ↖  ↑  ↗
  p4:Label, p5:Label, p6:Label, // ←  •  →
  p7:Label, p8:Label, p9:Label, // ↙  ↓  ↘
}

impl<F> ButtonL9<F>{
  /// Create a new button with a text label at the center (p5m other labels are blank, use `.addx` methods to fill them)
  pub fn new(                     label:impl Into<Label>, pad:Option<Insets>, callback:F) -> Self {
    let label = Label9 {
      p1:"".into(), p2:""   .into(), p3:"".into(), // ↖  ↑  ↗
      p4:"".into(), p5:label.into(), p6:"".into(), // ←  •  →
      p7:"".into(), p8:""   .into(), p9:"".into(), // ↙  ↓  ↘
    };
    let pad = Pad9 {
      p1:None, p2:None, p3:None, // ↖  ↑  ↗
      p4:None, p5:pad , p6:None, // ←  •  →
      p7:None, p8:None, p9:None, // ↙  ↓  ↘
    };
    let opt = LabelOpt{pad};
    Self {label, opt, callback}
  }
  /// Helper .methods for adding individual labels (add=center p5)
  pub fn add (mut self,         label:impl Into<Label>, pad:Option<Insets>) -> Self {self.label.p5 = label.into(); self.opt.pad.p5 = pad; self}
  pub fn add1(mut self,         label:impl Into<Label>, pad:Option<Insets>) -> Self {self.label.p1 = label.into(); self.opt.pad.p1 = pad; self}
  pub fn add2(mut self,         label:impl Into<Label>, pad:Option<Insets>) -> Self {self.label.p2 = label.into(); self.opt.pad.p2 = pad; self}
  pub fn add3(mut self,         label:impl Into<Label>, pad:Option<Insets>) -> Self {self.label.p3 = label.into(); self.opt.pad.p3 = pad; self}
  pub fn add4(mut self,         label:impl Into<Label>, pad:Option<Insets>) -> Self {self.label.p4 = label.into(); self.opt.pad.p4 = pad; self}
  pub fn add5(mut self,         label:impl Into<Label>, pad:Option<Insets>) -> Self {self.label.p5 = label.into(); self.opt.pad.p5 = pad; self}
  pub fn add6(mut self,         label:impl Into<Label>, pad:Option<Insets>) -> Self {self.label.p6 = label.into(); self.opt.pad.p6 = pad; self}
  pub fn add7(mut self,         label:impl Into<Label>, pad:Option<Insets>) -> Self {self.label.p7 = label.into(); self.opt.pad.p7 = pad; self}
  pub fn add8(mut self,         label:impl Into<Label>, pad:Option<Insets>) -> Self {self.label.p8 = label.into(); self.opt.pad.p8 = pad; self}
  pub fn add9(mut self,         label:impl Into<Label>, pad:Option<Insets>) -> Self {self.label.p9 = label.into(); self.opt.pad.p9 = pad; self}
  // pub fn add (mut self,         label:impl Into<Label>, pad:Option<Insets>) {self.addx(LPos::p5,label,pad)}
  /// Helper .method for adding a label to a given position (same as in [`LPos`])
  pub fn addx(mut self,idx:LPos,label:impl Into<Label>, pad:Option<Insets>) -> Self {match idx {
    LPos::p1 => {self.label.p1 = label.into(); self.opt.pad.p1 = pad}, //↖
    LPos::p2 => {self.label.p2 = label.into(); self.opt.pad.p2 = pad}, //↑
    LPos::p3 => {self.label.p3 = label.into(); self.opt.pad.p3 = pad}, //↗
    LPos::p4 => {self.label.p4 = label.into(); self.opt.pad.p4 = pad}, //←
    LPos::p5 => {self.label.p5 = label.into(); self.opt.pad.p5 = pad}, //•
    LPos::p6 => {self.label.p6 = label.into(); self.opt.pad.p6 = pad}, //→
    LPos::p7 => {self.label.p7 = label.into(); self.opt.pad.p7 = pad}, //↙
    LPos::p8 => {self.label.p8 = label.into(); self.opt.pad.p8 = pad}, //↓
    LPos::p9 => {self.label.p9 = label.into(); self.opt.pad.p9 = pad}, //↘
  } self }
}

const id_lvw1: ViewId = ViewId::new(1);
const id_lvw2: ViewId = ViewId::new(2);
const id_lvw3: ViewId = ViewId::new(3);
const id_lvw4: ViewId = ViewId::new(4);
const id_lvw5: ViewId = ViewId::new(5);
const id_lvw6: ViewId = ViewId::new(6);
const id_lvw7: ViewId = ViewId::new(7);
const id_lvw8: ViewId = ViewId::new(8);
const id_lvw9: ViewId = ViewId::new(9);

pub fn into_widget_pod(p:Pod<widgets::Label>) -> WidgetPod<widgets::Label> {
  WidgetPod::new_with_id_and_transform(p.widget, p.id, p.transform)
}

impl<F>              ViewMarker                 for ButtonL9<F> {}
impl<F,State,Action> View<State,Action,ViewCtx> for ButtonL9<F>
where F: Fn(&mut State, PointerB) -> MsgRes<Action> + Send + Sync + 'static {
  type Element = Pod<widget9::ButtonL9>;
  type ViewState = ();

  fn build(&self, ctx:&mut ViewCtx) -> (Self::Element, Self::ViewState) {
    // build based on LabelViews, which already implement build themselves:(Self::Element, Self::ViewState)
    let (child1,()) = ctx.with_id(id_lvw1, |ctx|{View::<State,Action,_>::build(&self.label.p1,ctx)});
    let (child2,()) = ctx.with_id(id_lvw2, |ctx|{View::<State,Action,_>::build(&self.label.p2,ctx)});
    let (child3,()) = ctx.with_id(id_lvw3, |ctx|{View::<State,Action,_>::build(&self.label.p3,ctx)});
    let (child4,()) = ctx.with_id(id_lvw4, |ctx|{View::<State,Action,_>::build(&self.label.p4,ctx)});
    let (child5,()) = ctx.with_id(id_lvw5, |ctx|{View::<State,Action,_>::build(&self.label.p5,ctx)});
    let (child6,()) = ctx.with_id(id_lvw6, |ctx|{View::<State,Action,_>::build(&self.label.p6,ctx)});
    let (child7,()) = ctx.with_id(id_lvw7, |ctx|{View::<State,Action,_>::build(&self.label.p7,ctx)});
    let (child8,()) = ctx.with_id(id_lvw8, |ctx|{View::<State,Action,_>::build(&self.label.p8,ctx)});
    let (child9,()) = ctx.with_id(id_lvw9, |ctx|{View::<State,Action,_>::build(&self.label.p9,ctx)});
    // pass built elements to the masonry widgets
    let label = [
      into_widget_pod(child1), into_widget_pod(child2), into_widget_pod(child3), // ↖  ↑  ↗
      into_widget_pod(child4), into_widget_pod(child5), into_widget_pod(child8), // ←  •  →
      into_widget_pod(child7), into_widget_pod(child6), into_widget_pod(child9), // ↙  ↓  ↘
      // child1.into_widget_pod(), child2.into_widget_pod(), child3.into_widget_pod(), // ↖  ↑  ↗
      // child4.into_widget_pod(), child5.into_widget_pod(), child8.into_widget_pod(), // ←  •  →
      // child7.into_widget_pod(), child6.into_widget_pod(), child9.into_widget_pod(), // ↙  ↓  ↘
    ];
    let pad = Pad9 {
      p1:self.opt.pad.p1.clone(), p2:self.opt.pad.p2.clone(), p3:self.opt.pad.p3.clone(), // ↖  ↑  ↗
      p4:self.opt.pad.p4.clone(), p5:self.opt.pad.p5.clone(), p6:self.opt.pad.p6.clone(), // ←  •  →
      p7:self.opt.pad.p7.clone(), p8:self.opt.pad.p8.clone(), p9:self.opt.pad.p9.clone(), // ↙  ↓  ↘
    };
    ctx.with_leaf_action_widget(|ctx| {ctx.new_pod(widget9::ButtonL9::from_label_pod(label,pad)) } )
  }

  fn rebuild(&self, prev:&Self, state:&mut Self::ViewState, ctx:&mut ViewCtx, mut el:Mut<Self::Element>) {
    // rebuild based on LabelViews, which already implement rebuild themselves (compare all the props)
    ctx.with_id(id_lvw1, |ctx|{View::<State,Action,_>::rebuild(&self.label.p1,&prev.label.p1,state,ctx, widget9::ButtonL9::label1_mut(&mut el));});
    ctx.with_id(id_lvw2, |ctx|{View::<State,Action,_>::rebuild(&self.label.p2,&prev.label.p2,state,ctx, widget9::ButtonL9::label2_mut(&mut el));});
    ctx.with_id(id_lvw3, |ctx|{View::<State,Action,_>::rebuild(&self.label.p3,&prev.label.p3,state,ctx, widget9::ButtonL9::label3_mut(&mut el));});
    ctx.with_id(id_lvw4, |ctx|{View::<State,Action,_>::rebuild(&self.label.p4,&prev.label.p4,state,ctx, widget9::ButtonL9::label4_mut(&mut el));});
    ctx.with_id(id_lvw5, |ctx|{View::<State,Action,_>::rebuild(&self.label.p5,&prev.label.p5,state,ctx, widget9::ButtonL9::label5_mut(&mut el));});
    ctx.with_id(id_lvw6, |ctx|{View::<State,Action,_>::rebuild(&self.label.p6,&prev.label.p6,state,ctx, widget9::ButtonL9::label6_mut(&mut el));});
    ctx.with_id(id_lvw7, |ctx|{View::<State,Action,_>::rebuild(&self.label.p7,&prev.label.p7,state,ctx, widget9::ButtonL9::label7_mut(&mut el));});
    ctx.with_id(id_lvw8, |ctx|{View::<State,Action,_>::rebuild(&self.label.p8,&prev.label.p8,state,ctx, widget9::ButtonL9::label8_mut(&mut el));});
    ctx.with_id(id_lvw9, |ctx|{View::<State,Action,_>::rebuild(&self.label.p9,&prev.label.p9,state,ctx, widget9::ButtonL9::label9_mut(&mut el));});

    // rebuild based on LabelOpt, do manuall diff for each prop
    if prev.opt.pad.p1 != self.opt.pad.p1 {widget9::ButtonL9::set_pad1 (&mut el, self.opt.pad.p1);}
    if prev.opt.pad.p2 != self.opt.pad.p2 {widget9::ButtonL9::set_pad2 (&mut el, self.opt.pad.p2);}
    if prev.opt.pad.p3 != self.opt.pad.p3 {widget9::ButtonL9::set_pad3 (&mut el, self.opt.pad.p3);}
    if prev.opt.pad.p4 != self.opt.pad.p4 {widget9::ButtonL9::set_pad4 (&mut el, self.opt.pad.p4);}
    if prev.opt.pad.p5 != self.opt.pad.p5 {widget9::ButtonL9::set_pad5 (&mut el, self.opt.pad.p5);}
    if prev.opt.pad.p6 != self.opt.pad.p6 {widget9::ButtonL9::set_pad6 (&mut el, self.opt.pad.p6);}
    if prev.opt.pad.p7 != self.opt.pad.p7 {widget9::ButtonL9::set_pad7 (&mut el, self.opt.pad.p7);}
    if prev.opt.pad.p8 != self.opt.pad.p8 {widget9::ButtonL9::set_pad8 (&mut el, self.opt.pad.p8);}
    if prev.opt.pad.p9 != self.opt.pad.p9 {widget9::ButtonL9::set_pad9 (&mut el, self.opt.pad.p9);}
  }

  fn teardown(&self, _:&mut Self::ViewState, ctx:&mut ViewCtx, mut el:Mut<Self::Element>) {
    // teardown LabelViews, which already implement teardown themselves
    ctx.with_id(id_lvw1, |ctx|{View::<State,Action,_>::teardown(&self.label.p1,&mut (),ctx, widget9::ButtonL9::label1_mut(&mut el));});
    ctx.with_id(id_lvw2, |ctx|{View::<State,Action,_>::teardown(&self.label.p2,&mut (),ctx, widget9::ButtonL9::label1_mut(&mut el));});
    ctx.with_id(id_lvw3, |ctx|{View::<State,Action,_>::teardown(&self.label.p3,&mut (),ctx, widget9::ButtonL9::label1_mut(&mut el));});
    ctx.with_id(id_lvw4, |ctx|{View::<State,Action,_>::teardown(&self.label.p4,&mut (),ctx, widget9::ButtonL9::label1_mut(&mut el));});
    ctx.with_id(id_lvw5, |ctx|{View::<State,Action,_>::teardown(&self.label.p5,&mut (),ctx, widget9::ButtonL9::label1_mut(&mut el));});
    ctx.with_id(id_lvw6, |ctx|{View::<State,Action,_>::teardown(&self.label.p6,&mut (),ctx, widget9::ButtonL9::label1_mut(&mut el));});
    ctx.with_id(id_lvw7, |ctx|{View::<State,Action,_>::teardown(&self.label.p7,&mut (),ctx, widget9::ButtonL9::label1_mut(&mut el));});
    ctx.with_id(id_lvw8, |ctx|{View::<State,Action,_>::teardown(&self.label.p8,&mut (),ctx, widget9::ButtonL9::label1_mut(&mut el));});
    ctx.with_id(id_lvw9, |ctx|{View::<State,Action,_>::teardown(&self.label.p9,&mut (),ctx, widget9::ButtonL9::label1_mut(&mut el));});
    ctx.teardown_leaf(el); // teardown the button element itself
  }

  fn message(&self, _:&mut Self::ViewState, id_path:&[ViewId], message:DynMessage, app_state:&mut State) -> MsgRes<Action> {
    match id_path.split_first() {
      Some((&id_lvw1, rest)) => self.label.p1.message(&mut(), rest, message, app_state),
      Some((&id_lvw2, rest)) => self.label.p2.message(&mut(), rest, message, app_state),
      Some((&id_lvw3, rest)) => self.label.p3.message(&mut(), rest, message, app_state),
      Some((&id_lvw4, rest)) => self.label.p4.message(&mut(), rest, message, app_state),
      Some((&id_lvw5, rest)) => self.label.p5.message(&mut(), rest, message, app_state),
      Some((&id_lvw6, rest)) => self.label.p6.message(&mut(), rest, message, app_state),
      Some((&id_lvw7, rest)) => self.label.p7.message(&mut(), rest, message, app_state),
      Some((&id_lvw8, rest)) => self.label.p8.message(&mut(), rest, message, app_state),
      Some((&id_lvw9, rest)) => self.label.p9.message(&mut(), rest, message, app_state),
      None => match message.downcast::<masonry::core::Action>() {
        Ok(action)   => {
          if let masonry::core::Action::ButtonPressed(button) = *action {
            (self.callback)(app_state, button)
          } else        {tracing::error!("Wrong action type in ButtonL9::message: {action:?}");
            MessageResult::Stale(action)} }
        Err(message) => {tracing::error!("Wrong message type in ButtonL9::message: {message:?}");
            MessageResult::Stale(message) }   },
      _    =>           {tracing::warn! ("Got unexpected ID path in ButtonL9::message");
            MessageResult::Stale(message)      }
    }
  }
}
