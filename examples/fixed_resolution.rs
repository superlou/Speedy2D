/*
 *  Copyright 2021 QuantumBadger
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

#![deny(warnings)]

use speedy2d::color::Color;
use speedy2d::dimen::UVec2;
use speedy2d::font::{Font, FormattedTextBlock, TextLayout, TextOptions};
use speedy2d::window::{WindowHandler, WindowHelper, WindowCreationOptions, WindowSize};
use speedy2d::{Graphics2D, Window};

fn main()
{
    simple_logger::SimpleLogger::new().init().unwrap();

    let options =  WindowCreationOptions::new_windowed(
        WindowSize::PhysicalPixels((640, 240).into()),
        None,
    ).with_fixed_resolution(true);
    
    let window = Window::new_with_options("Fixed resolution window", options).unwrap();

    let font = Font::new(include_bytes!("../assets/fonts/NotoSans-Regular.ttf")).unwrap();
    let text = font.layout_text("Hello world!", 64.0, TextOptions::new());

    window.run_loop(MyWindowHandler {
        text,
        new_resolution: None
    });
}

struct MyWindowHandler
{
    text: FormattedTextBlock,
    new_resolution: Option<UVec2>,
}

impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        if let Some(resolution) = self.new_resolution {
            graphics.set_resolution(resolution);
            self.new_resolution = None;
        }

        graphics.clear_screen(Color::WHITE);
        graphics.draw_circle((150.0, 120.0), 75.0, Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_text((290.0, 90.0), Color::BLACK, &self.text);
        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
    
    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        _virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode
    ) {
        self.new_resolution = Some(helper.get_size_pixels());
    }
}
