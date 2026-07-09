use gpui::*;
use gpui_component::{
    button::*,
    table::{Table, TableBody, TableCell, TableFooter, TableHead, TableHeader, TableRow},
    *,
};

pub struct TaoKanban;
impl Render for TaoKanban {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_start()
            //.child("Hello, World!")
            //.child(
            //    Button::new("ok")
            //        .primary()
            //        .label("Let's Go!")
            //        .on_click(|_, _, _| println!("Clicked!")),
            //)
            .child(
                Table::new()
                    .child(
                        TableHeader::new().child(
                            TableRow::new()
                                .child(TableHead::new().child("姓名"))
                                .child(TableHead::new().child("任务名称"))
                                .child(TableHead::new().text_right().child("状态")),
                        ),
                    )
                    .child(
                        TableBody::new().child(
                            TableRow::new()
                                .child(TableCell::new().child("INV001"))
                                .child(TableCell::new().child("Paid"))
                                .child(TableCell::new().text_right().child("$250.00")),
                        ),
                    )
                    .child(
                        TableFooter::new().child(
                            TableRow::new()
                                .child(TableCell::new().child("Total"))
                                .child(TableCell::new().child(""))
                                .child(TableCell::new().text_right().child("$250.00")),
                        ),
                    ),
            )
    }
}

fn main() {
    gpui_platform::application().run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            let mut window_opts = WindowOptions::default();
            window_opts.is_movable = false;
            window_opts.is_minimizable = false;
            cx.open_window(window_opts, |window, cx| {
                let view = cx.new(|_| TaoKanban);
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
