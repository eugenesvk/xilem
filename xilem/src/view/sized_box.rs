// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use masonry::widget;
use vello::kurbo::RoundedRectRadii;
use vello::peniko::{Brush, Color};
use xilem_core::ViewMarker;


use crate::{
    core::{Mut, View, ViewId},
    Pod, ViewCtx, WidgetView,
};
use xilem_colors::tokens::Token;

/// A widget with predefined size.
///
/// This widget forces its child to have a specific width and/or height (assuming values are permitted by
/// this widget's parent). If either the width or height is not set, this widget will size itself
/// to match the child's size in that dimension.
pub fn sized_box<State, Action, V>(inner: V) -> SizedBox<V, State, Action>
where
    V: WidgetView<State, Action>,
{
    SizedBox {
        inner,
        height: None,
        width: None,
        background: None,
        //border: None,
        border_color: None,
        border_width: 0.,
        corner_radius: RoundedRectRadii::from_single_radius(0.0),
        phantom: PhantomData,
    }
}

pub struct SizedBox<V, State, Action = ()> {
    inner: V,
    width: Option<f64>,
    height: Option<f64>,
    background: Option<Brush>,
    border_color: Option<Token>,
    border_width: f64,
    //border: Option<BorderStyle>,
    corner_radius: RoundedRectRadii,
    phantom: PhantomData<fn() -> (State, Action)>,
}

impl<V, State, Action> SizedBox<V, State, Action> {
    /// Set container's width.
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set container's height.
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Expand container to fit the parent.
    ///
    /// Only call this method if you want your widget to occupy all available
    /// space. If you only care about expanding in one of width or height, use
    /// [`expand_width`] or [`expand_height`] instead.
    ///
    /// [`expand_height`]: Self::expand_height
    /// [`expand_width`]: Self::expand_width
    pub fn expand(mut self) -> Self {
        self.width = Some(f64::INFINITY);
        self.height = Some(f64::INFINITY);
        self
    }

    /// Expand the container on the x-axis.
    ///
    /// This will force the child to have maximum width.
    pub fn expand_width(mut self) -> Self {
        self.width = Some(f64::INFINITY);
        self
    }

    /// Expand the container on the y-axis.
    ///
    /// This will force the child to have maximum height.
    pub fn expand_height(mut self) -> Self {
        self.height = Some(f64::INFINITY);
        self
    }

    /// Builder-style method for setting the background for this widget.
    ///
    /// This can be passed anything which can be represented by a [`Brush`];
    /// notably, it can be any [`Color`], any gradient, or an [`Image`].
    ///
    /// [`Image`]: vello::peniko::Image
    pub fn background(mut self, brush: impl Into<Brush>) -> Self {
        self.background = Some(brush.into());
        self
    }

    /// Builder-style method for painting a border around the widget with a color and width.
    pub fn border(mut self, color: Option<Token>, width: impl Into<f64>) -> Self {
        self.border_color = color;
        self.border_width = width.into();
        self
    }

    /// Builder style method for rounding off corners of this container by setting a corner radius
    pub fn rounded(mut self, radius: impl Into<RoundedRectRadii>) -> Self {
        self.corner_radius = radius.into();
        self
    }
}

impl<V, State, Action> ViewMarker for SizedBox<V, State, Action> {}
impl<V, State, Action> View<State, Action, ViewCtx> for SizedBox<V, State, Action>
where
    State: 'static,
    Action: 'static,
    V: WidgetView<State, Action>,
{
    type Element = Pod<widget::SizedBox>;
    type ViewState = V::ViewState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let (child, child_state) = self.inner.build(ctx);
        let mut widget = widget::SizedBox::new_pod(child.inner.boxed())
            .raw_width(self.width)
            .raw_height(self.height)
            .rounded(self.corner_radius);
        if let Some(background) = &self.background {
            widget = widget.background(background.clone());
        }
        if let Some(_) = &self.border_color {
            widget = widget.border(self.border_color, self.border_width);
        }
        (ctx.new_pod(widget), child_state)
    }

    fn rebuild<'el>(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'el, Self::Element>,
    ) -> Mut<'el, Self::Element> {
        if self.width != prev.width {
            match self.width {
                Some(width) => element.set_width(width),
                None => element.unset_width(),
            }
        }
        if self.height != prev.height {
            match self.height {
                Some(height) => element.set_height(height),
                None => element.unset_height(),
            }
        }
        if self.background != prev.background {
            match &self.background {
                Some(background) => element.set_background(background.clone()),
                None => element.clear_background(),
            }
        }
        if self.border_color != prev.border_color {
            match &self.border_color {
                Some(border) => element.set_border(Some(*border), self.border_width),
                None => element.clear_border(),
            }
        }
        if self.corner_radius != prev.corner_radius {
            element.set_rounded(self.corner_radius);
        }
        {
            let mut child = element
                .child_mut()
                .expect("We only create SizedBox with a child");
            self.inner
                .rebuild(&prev.inner, view_state, ctx, child.downcast());
        }
        element
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
    ) {
        let mut child = element
            .child_mut()
            .expect("We only create SizedBox with a child");
        self.inner.teardown(view_state, ctx, child.downcast());
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        message: xilem_core::DynMessage,
        app_state: &mut State,
    ) -> crate::MessageResult<Action> {
        self.inner.message(view_state, id_path, message, app_state)
    }
}

/// Something that can be used as the border for a widget.
#[derive(PartialEq)]
struct _BorderStyle {
    width: f64,
    color: Color,
}
