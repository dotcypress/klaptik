pub use paste::paste;

#[macro_export]
macro_rules! widget_group {
(
    $name:ident<$state:ty>,
    {$(
        $node_name:ident: $widget_type:ty $(, $widget_args:expr)*;
    )+}
    $(, $update_hook:expr)?
) => {
    pub struct $name {
        pub $($node_name: $widget_type,)+
    }

    impl $name {
        pub fn new() -> Self {
            Self {
                $(
                    $node_name: <$widget_type>::new($($widget_args),*),
                )+
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
                self.$node_name.invalidate();
            )+
        }

        fn render<C: Canvas>(&mut self, canvas: &mut C) {
            $(
                self.$node_name.render(canvas);
            )+
        }
    }
}
;}

#[macro_export]
macro_rules! widget_mux {
(
    $name:ident<$state:ty>,
    $active:expr,
    {$(
        $node_name:ident: $widget_type:ty $(, $widget_args:expr)*;
    )+}
    $(, $update_hook:expr)?
) => {
    paste! {
        #[derive(PartialEq, Clone, Copy, Debug)]
        pub enum [<$name:camel Node>] {
            $([<$node_name:camel>]),+
        }

        pub struct $name {
            active: [<$name:camel Node>],
            pub $($node_name: $widget_type,)+
        }

        impl $name {
            pub fn new() -> Self {
                let mut view = Self {
                    active: $active,
                    $(
                        $node_name: <$widget_type>::new($($widget_args),*),
                    )+
                };
                view
            }

            pub fn set_active(&mut self, node: [<$name:camel Node>]) {
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
                    self.$node_name.invalidate();
                )+
            }

            fn render<C: Canvas>(&mut self, canvas: &mut C) {
                $(
                    if self.active == [<$name:camel Node>]::[<$node_name:camel>] {
                        self.$node_name.render(canvas);
                    }
                )+
            }
        }
    }
}
;}
