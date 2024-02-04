mod modules;
use modules::ani::get_ani_schedule;

use modules::structs::AniData;

use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, Submenu},
    TrayIconBuilder, TrayIconEvent,
};
use chrono::Local;

fn main() {
    let path = "./image/logo_128x128.png";
    let icon = load_icon(std::path::Path::new(path));

    let tray_menu = get_tray_menu().unwrap();

    let quit_i = MenuItem::new("종료", true, None);
    tray_menu.append(&quit_i);

    let mut tray_icon = 
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("")
            .with_icon(icon)
            .build()
            .unwrap();

    let last_update = Local::now().timestamp();

    let event_loop = EventLoopBuilder::new().build();

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // 종료
        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_i.id() {
                // tray_icon.take();
                *control_flow = ControlFlow::Exit;
            }
            // println!("{event:?}");
        }

        if let Ok(event) = tray_channel.try_recv() {
            if last_update + 86400 < Local::now().timestamp() {
                let mut tray_menu: Option<Box<dyn tray_icon::menu::ContextMenu>> = None;

                let temp = get_tray_menu().unwrap();
                temp.append(&quit_i);
                tray_menu = Some(Box::new(temp));

                tray_icon.set_menu(tray_menu);
                println!("update ani list")
            }
            // println!("B {event:?}");
        }
    })
}

fn get_tray_menu() -> Result<Menu, reqwest::Error>{
    let tray_menu = Menu::new();

    let wod = vec![
        "일요일",
        "월요일",
        "화요일",
        "수요일",
        "목요일",
        "금요일",
        "토요일",
    ];

    for (i, day) in wod.iter().enumerate() {
        let ani_list = get_ani_schedule(i as i32);
        match ani_list {
            Ok(ani_list) => {
                let sub_menu = Submenu::new(day, true);
                for ani in ani_list {
                    sub_menu.append(&MenuItem::new(ani, true, None));
                }
                tray_menu.append(&sub_menu);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(tray_menu)
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}