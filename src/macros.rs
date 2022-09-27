pub use paste::paste;

#[macro_export]
macro_rules! widget {
(
    $name:ident<$state:ty>,
    nodes: {$(
        $widget_name:ident: $widget_type:ty $(, $widget_args:expr)*;
    )+}
    $(, active: $active:ident)?
    $(, update: $update_hook:expr)?
) => {
    paste! {
        #[derive(PartialEq, Clone, Copy, Debug)]
        pub enum [<$name:camel Node>] {
            $([<$widget_name:camel>]),+
        }

        pub struct $name {
            active: Option<[<$name:camel Node>]>,
            pub $($widget_name: $widget_type,)+
        }

        impl $name {
            pub fn new() -> Self {
                let mut view = Self {
                    active: None,
                    $(
                        $widget_name: <$widget_type>::new($($widget_args),*),
                    )+
                };
                $(
                    view.active = Some([<$name:camel Node>]::[<$active:camel>]);
                )?
                view
            }

            pub fn set_active(&mut self, node: Option<[<$name:camel Node>]>) {
                if self.active != node {
                    self.active = node;
                    self.invalidate();
                }
            }
        }

        impl Widget<$state> for $name {
            fn update(&mut self, state: $state) {
                $(
                    $update_hook(self, state);
                )?
            }

            fn invalidate(&mut self) {
                $(
                    self.$widget_name.invalidate();
                )+
            }

            fn render<C: Canvas>(&mut self, canvas: &mut C) {
                $(
                    if matches!(self.active, None | Some([<$name:camel Node>]::[<$widget_name:camel>])) {
                        self.$widget_name.render(canvas);
                    }
                )+
            }
        }
    }
}
;}
