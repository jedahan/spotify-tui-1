use super::super::app::{ActiveBlock, App, RouteId, LIBRARY_OPTIONS};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &LIBRARY_OPTIONS,
                Some(app.library.selected_index),
            );
            app.library.selected_index = next_index;
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &LIBRARY_OPTIONS,
                Some(app.library.selected_index),
            );
            app.library.selected_index = next_index;
        }
        // `library` should probably be an array of structs with enums rather than just using indexes
        // like this
        Key::Char('\n') => match app.library.selected_index {
            // Made For You,
            0 => {
                app.push_navigation_stack(RouteId::MadeForYou, ActiveBlock::MadeForYou);
            }
            // Recently Played,
            1 => {
                if let Some(spotify) = &app.spotify {
                    match spotify
                        // Seems I need to clone here becuase `current_user_recently_played`
                        // consumes `self`?
                        .clone()
                        .current_user_recently_played(app.large_search_limit)
                    {
                        Ok(result) => {
                            app.recently_played.result = Some(result);
                            app.push_navigation_stack(
                                RouteId::RecentlyPlayed,
                                ActiveBlock::RecentlyPlayed,
                            );
                        }
                        Err(e) => {
                            app.handle_error(e);
                        }
                    }
                };
            }
            // Liked Songs,
            2 => {
                app.get_current_user_saved_tracks(None);
            }
            // Albums,
            3 => {
                if let Some(spotify) = &app.spotify {
                    match spotify.current_user_saved_albums(app.large_search_limit, 0) {
                        Ok(result) => {
                            app.library.saved_albums.add_pages(result);
                            app.push_navigation_stack(RouteId::AlbumList, ActiveBlock::AlbumList);
                        }
                        Err(e) => {
                            app.handle_error(e);
                        }
                    }
                };
            }
            //  Artists,
            4 => {
                app.get_artists(None);
            }
            // Podcasts,
            5 => {
                app.push_navigation_stack(RouteId::Podcasts, ActiveBlock::Podcasts);
            }
            // This is required because Rust can't tell if this pattern in exhaustive
            _ => {}
        },
        _ => (),
    };
}
