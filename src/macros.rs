pub use paste::paste;

#[macro_export]
macro_rules! widget {
(
    $name:ident<$state:ty>,
    nodes: {$(
        $widget_name:ident ($widget_type:ty $(, $widget_args:expr)*)
    )+}
    $(, active_node: $active_node:ident)?
    $(, set_state: $set_state_hook:expr)?
) => {
    paste! {
        #[derive(PartialEq, Clone, Copy, Debug)]
        pub enum [<$name:camel Node>] {
            $([<$widget_name:camel>]),+
        }

        pub struct $name {
            active_node: Option<[<$name:camel Node>]>,
            pub $($widget_name: $widget_type,)+
        }

        impl $name {
            pub fn new() -> Self {
                let mut view = Self {
                    active_node: None,
                    $(
                        $widget_name: <$widget_type>::new($($widget_args),*),
                    )+
                };
                $(
                    view.active_node = Some([<$name:camel Node>]::[<$active_node:camel>]);
                )?
                view
            }

            pub fn set_active(&mut self, widget: Option<[<$name:camel Node>]>) {
                if self.active_node != widget {
                    self.active_node = widget;
                    self.invalidate();
                }
            }
        }

        impl Widget<$state> for $name {
            fn set_state(&mut self, state: $state) {
                $(
                    $set_state_hook(self, state);
                )?
            }

            fn invalidate(&mut self) {
                $(
                    self.$widget_name.invalidate();
                )+
            }

            fn render<C: Canvas>(&mut self, canvas: &mut C) {
                $(
                    if matches!(self.active_node, None | Some([<$name:camel Node>]::[<$widget_name:camel>])) {
                        self.$widget_name.render(canvas);
                    }
                )+
            }
        }
    }
}
;}
