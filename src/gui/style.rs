use macroquad::{
    color::Color,
    math::RectOffset,
    texture::Image,
    ui::{root_ui, Skin},
    color,
};

const NO_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.0);

pub struct SkinCollection {
    pub main_menu_skin: Skin,
    pub login_skin: Skin,
    pub authenticating_skin: Skin,
    pub error_skin: Skin,
    pub editor_skin: Skin,
    pub editor_window_header_skin: Skin,
    pub editor_menu_skin: Skin,
    pub editor_menu_selected_skin: Skin,
    pub editor_context_menu_skin: Skin,
    pub editor_menu_bg: Skin,
    pub editor_menu_header_bg: Skin,
    pub cheat_skin: Skin,
}

impl SkinCollection {
    pub fn new() -> SkinCollection {
        let main_menu_skin = {
            let label_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/ui/MinimalPixel v2.ttf"))
                .unwrap()
                .text_color(Color::from_rgba(255, 255, 255, 255))
                .font_size(130)
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/preview_background_2.png"),
                    None,
                ))
                .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
                .margin(RectOffset::new(-40.0, -40.0, -40.0, -40.0))
                .background_hovered(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/preview_background_2.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/preview_background_2.png"),
                    None,
                ))
                .font(include_bytes!("../../assets/ui/MinimalPixel v2.ttf"))
                .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .reverse_background_z(true)
                .font_size(45)
                .build();

            Skin {
                label_style,
                button_style,
                ..root_ui().default_skin()
            }
        };

        let login_skin = {
            let label_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/ui/MinimalPixel v2.ttf"))
                .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(20)
                .build();

            let window_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/window_background_2.png"),
                    None,
                ))
                .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
                .margin(RectOffset::new(-30.0, -30.0, -30.0, -30.0))
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/button_background_2.png"),
                    None,
                ))
                .background_margin(RectOffset::new(8.0, 8.0, 12.0, 12.0))
                .background_hovered(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/button_hovered_background_2.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/button_clicked_background_2.png"),
                    None,
                ))
                .font(include_bytes!("../../assets/ui/MinimalPixel v2.ttf"))
                .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(25)
                .build();

            let tabbar_style = root_ui()
                .style_builder()
                .background_margin(RectOffset::new(8.0, 8.0, 12.0, 12.0))
                .font(include_bytes!("../../assets/ui/MinimalPixel v2.ttf"))
                .unwrap()
                .color(Color::from_rgba(58, 68, 102, 255))
                .color_hovered(Color::from_rgba(149, 165, 190, 255))
                .color_clicked(Color::from_rgba(129, 145, 170, 255))
                .color_selected(Color::from_rgba(139, 155, 180, 255))
                .color_selected_hovered(Color::from_rgba(149, 165, 190, 255))
                .text_color(Color::from_rgba(255, 255, 255, 255))
                .font_size(20)
                .build();

            let editbox_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/editbox_background2.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/editbox_background.png"),
                    None,
                ))
                .font(include_bytes!("../../assets/ui/MinimalPixel v2.ttf"))
                .unwrap()
                .background_margin(RectOffset::new(2., 2., 2., 2.))
                .text_color(Color::from_rgba(120, 120, 120, 255))
                .font_size(20)
                .build();
            let checkbox_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/checkbox_background.png"),
                    None,
                ))
                .background_hovered(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/checkbox_hovered_background.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/checkbox_clicked_background.png"),
                    None,
                ))
                .build();
            let combobox_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/combobox_background.png"),
                    None,
                ))
                .background_margin(RectOffset::new(4., 25., 6., 6.))
                .font(include_bytes!("../../assets/ui/MinimalPixel v2.ttf"))
                .unwrap()
                .text_color(Color::from_rgba(120, 120, 120, 255))
                .color(Color::from_rgba(210, 210, 210, 255))
                .font_size(25)
                .build();

            Skin {
                label_style,
                button_style,
                tabbar_style,
                window_style,
                editbox_style,
                combobox_style,
                checkbox_style,
                ..root_ui().default_skin()
            }
        };

        let authenticating_skin = {
            let label_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/ui/MinimalPixel v2.ttf"))
                .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(35)
                .build();

            Skin {
                label_style,
                ..root_ui().default_skin()
            }
        };

        let error_skin = {
            let label_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/ui/MinimalPixel v2.ttf"))
                .unwrap()
                .text_color(Color::from_rgba(255, 0, 0, 255))
                .font_size(20)
                .build();

            Skin {
                label_style,
                ..root_ui().default_skin()
            }
        };

        let editor_skin = {
            let window_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/window_background_2.png"),
                    None,
                ))
                .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
                .margin(RectOffset::new(-30.0, -30.0, -30.0, -30.0))
                .build();

            let group_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/window_background_2.png"),
                    None,
                ))
                .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .color(NO_COLOR)
                .color_hovered(NO_COLOR)
                .color_clicked(NO_COLOR)
                .build();

            let label_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(8.0, 8.0, 4.0, 4.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(16)
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/button_background_2.png"),
                    None,
                ))
                .background_margin(RectOffset::new(8.0, 8.0, 12.0, 12.0))
                .margin(RectOffset::new(8.0, 8.0, -4.0, -4.0))
                .background_hovered(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/button_hovered_background_2.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/button_clicked_background_2.png"),
                    None,
                ))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(20)
                .build();

            let editbox_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/editbox_background2.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/editbox_background.png"),
                    None,
                ))
                .background_margin(RectOffset::new(2., 2., 2., 2.))
                .margin(RectOffset::new(0.0,0.0,4.0,4.0))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(22)
                .build();

            let checkbox_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/checkbox_background.png"),
                    None,
                ))
                .background_hovered(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/checkbox_hovered_background.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/checkbox_clicked_background.png"),
                    None,
                ))
                .build();

            let combobox_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/combobox_background.png"),
                    None,
                ))
                .background_margin(RectOffset::new(4., 25., 6., 6.))
                .text_color(Color::from_rgba(120, 120, 120, 255))
                .color(Color::from_rgba(210, 210, 210, 255))
                .font_size(22)
                .build();

            Skin {
                window_style,
                group_style,
                label_style,
                button_style,
                editbox_style,
                checkbox_style,
                combobox_style,
                ..root_ui().default_skin()
            }
        };

        let editor_window_header_skin = {
            let label_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(8.0, 8.0, 4.0, 16.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(22)
                .build();

            Skin {
                label_style,
                ..editor_skin.clone()
            }
        };

        let editor_menu_skin = {
            let label_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(8.0, 8.0, 4.0, 4.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(16)
                .build();

            let button_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .color(Color::from_rgba(58, 68, 68, 255))
                .color_hovered(Color::from_rgba(58, 68, 102, 255))
                .color_clicked(Color::from_rgba(58, 68, 68, 255))
                .build();

            let scrollbar_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(38, 43, 68, 255))
                .build();

            let scrollbar_handle_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(58, 68, 68, 255))
                .color_hovered(Color::from_rgba(58, 68, 102, 255))
                .color_clicked(Color::from_rgba(58, 68, 68, 255))
                .build();

            Skin {
                label_style,
                button_style,
                scrollbar_style,
                scrollbar_handle_style,
                ..editor_skin.clone()
            }
        };

        let editor_menu_selected_skin = {
            let label_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(8.0, 8.0, 4.0, 4.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(16)
                .build();

            let button_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .color(Color::from_rgba(58, 68, 102, 255))
                .color_hovered(Color::from_rgba(58, 68, 68, 255))
                .color_clicked(Color::from_rgba(58, 68, 102, 255))
                .build();

            Skin {
                label_style,
                button_style,
                ..editor_menu_skin.clone()
            }
        };

        let editor_context_menu_skin = {
            let button_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .color(Color::from_rgba(38, 43, 68, 255))
                .color_hovered(Color::from_rgba(38, 43, 102, 255))
                .color_clicked(Color::from_rgba(38, 43, 68, 255))
                .build();

            let scrollbar_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(0, 0, 0, 0))
                .build();

            let scrollbar_handle_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(0, 0, 0, 0))
                .color_hovered(Color::from_rgba(0, 0, 0, 0))
                .color_clicked(Color::from_rgba(0, 0, 0, 0))
                .build();

            Skin {
                button_style,
                scrollbar_style,
                scrollbar_handle_style,
                ..editor_menu_skin.clone()
            }
        };

        let editor_menu_bg = {
            let button_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .color(Color::from_rgba(58, 68, 68, 255))
                .color_hovered(Color::from_rgba(58, 68, 68, 255))
                .color_clicked(Color::from_rgba(58, 68, 68, 255))
                .build();

            Skin {
                button_style,
                ..editor_skin.clone()
            }
        };

        let editor_menu_header_bg = {
            let button_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .color(Color::from_rgba(38, 43, 68, 255))
                .color_hovered(Color::from_rgba(38, 43, 68, 255))
                .color_clicked(Color::from_rgba(38, 43, 68, 255))
                .build();

            Skin {
                button_style,
                ..editor_skin.clone()
            }
        };

        let cheat_skin = root_ui().default_skin();

        SkinCollection {
            main_menu_skin,
            login_skin,
            authenticating_skin,
            error_skin,
            editor_skin,
            editor_window_header_skin,
            editor_menu_skin,
            editor_menu_selected_skin,
            editor_context_menu_skin,
            editor_menu_bg,
            editor_menu_header_bg,
            cheat_skin,
        }
    }
}
