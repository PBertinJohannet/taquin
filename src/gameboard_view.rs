//! Gameboard view.
use graphics::character::CharacterCache;
use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::{Image, Line, Rectangle, Transformed};

use GameboardController;

/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
    /// Selected cell background color.
    pub selected_cell_background_color: Color,
    /// Text color.
    pub text_color: Color,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 800.0,
            background_color: [0.2, 0.2, 0.3, 1.0],
            border_color: [0.0, 0.0, 0.0, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            text_color: [1.0, 1.0, 0.1, 1.0],
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView { settings: settings }
    }

    /// Draws the grid on the screen.
    pub fn draw_grid<G: Graphics>(&self, settings: &GameboardViewSettings, c: &Context, g: &mut G) {

        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.size,
            settings.size,
        ];
        // Draw board background.
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        // Draw section borders.
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);
        for i in 0..::SIZE.0 {
            // Set up coordinates.
            let x = settings.position[0] + i as f64 / (::SIZE.0 as f64) * settings.size;
            let y = settings.position[1] + i as f64 / (::SIZE.0 as f64) * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            section_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            section_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);


    }

    /// Draw selected cell.
    pub fn draw_selected_cell<G: Graphics>(
        &self,
        controller: &GameboardController,
        c: &Context,
        g: &mut G,
        settings: &GameboardViewSettings,
    ) {
        let cell_size = settings.size / (::SIZE.0 as f64);
        let pos = [
            controller.gameboard.y as f64 * cell_size,
            controller.gameboard.x as f64 * cell_size,
        ];
        let cell_rect = [
            settings.position[0] + pos[0],
            settings.position[1] + pos[1],
            cell_size,
            cell_size,
        ];
        Rectangle::new(settings.selected_cell_background_color)
            .draw(cell_rect, &c.draw_state, c.transform, g);
    }



    /// Draw a string in a cell.
    pub fn draw_in_cell<G: Graphics, C>(
        &self,
        controller: &GameboardController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
        settings: &GameboardViewSettings,
        (i, j): (usize, usize),
        ch: usize,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        let str = ch.to_string();
        let text_image = Image::new_color(settings.text_color);
        let cell_size = settings.size / (::SIZE.0 as f64);
        let font_size = self.settings.size as u32 / ((::SIZE.0 *2) as u32);
        let pos = [
            settings.position[0] + i as f64 * cell_size + font_size as f64 / 2.0 -
                0.5 * (font_size * (str.len() - 1) as u32) as f64 + font_size as f64  ,
            settings.position[1] + j as f64 * cell_size + font_size as f64,
        ];
        for (index, char) in str.chars().enumerate() {
            if let Ok(character) = glyphs.character(font_size, char) {
                let ch_x = pos[0] + character.left() + font_size as f64 * 0.5 * index as f64;
                let ch_y = pos[1] - character.top();
                text_image.draw(
                    character.texture,
                    &c.draw_state,
                    c.transform.trans(ch_x, ch_y),
                    g,
                );
            }
        }
    }


    /// Draw cells.
    pub fn draw_cells<G: Graphics, C>(
        &self,
        settings: &GameboardViewSettings,
        controller: &GameboardController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        // Draw characters.
        for j in 0..::SIZE.0 {
            for i in 0..::SIZE.1 {
                if let Some(ch) = controller.gameboard.at((i, j)) {
                    self.draw_in_cell(controller, glyphs, c, g, settings, (i, j), ch);
                }
            }
        }
    }




    /// Draw gameboard.
    pub fn draw<G: Graphics, C>(
        &self,
        controller: &GameboardController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        let ref settings = self.settings;

        self.draw_grid(settings, c, g);

        self.draw_selected_cell(controller, c, g, settings);

        self.draw_cells(settings, controller, glyphs, c, g);
    }
}
