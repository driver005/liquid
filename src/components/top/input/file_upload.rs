use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct FileUpload {}
impl FileUpload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file_upload(self, files: RwSignal<Vec<String>>, theme: Theme) -> impl View {
        let dropzone = floem::views::Stack::vertical((
            floem::views::Label::new("📁").style(move |s| s.apply(theme.file_upload_icon_style())),
            floem::views::Label::new("Drop files here or click to browse")
                .style(move |s| s.apply(theme.file_upload_title_style())),
            floem::views::Label::new("Supports any file type")
                .style(move |s| s.apply(theme.file_upload_subtitle_style())),
        ))
        .style(move |s| s.apply(theme.file_upload_zone_style(false)).height(200.0))
        .on_event_stop(floem::event::listener::Click, move |_, _| {
            files.update(|f| f.push(format!("file_{}.txt", f.len() + 1)));
        });

        let file_list = floem::views::dyn_stack(
            move || files.get(),
            |f| f.clone(),
            move |f| {
                let files_sig = files;
                let fname = f.clone();
                floem::views::Stack::horizontal((
                    floem::views::Label::new("📄")
                        .style(move |s| s.apply(theme.file_upload_item_icon_style())),
                    floem::views::Label::new(fname.clone())
                        .style(move |s| s.apply(theme.file_upload_item_name_style())),
                    floem::views::Label::new("✕")
                        .style({
                            move |s| {
                                s.apply(theme.file_upload_item_delete_style()).hover({
                                    move |s| s.apply(theme.file_upload_item_delete_hover_style())
                                })
                            }
                        })
                        .on_event_stop(floem::event::listener::Click, move |_, _| {
                            files_sig.update(|f| f.retain(|x| x != &fname));
                        }),
                ))
                .style(move |s| s.apply(theme.file_upload_item_container_style()))
            },
        );

        floem::views::Stack::vertical((
            dropzone,
            file_list.style(move |s| s.apply(theme.file_upload_list_container_style())),
        ))
        .style(move |s| s.apply(theme.file_upload_container_style()))
    }
}
