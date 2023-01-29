use clap::{Parser, Subcommand, ValueEnum};
use knossos::{
    maze::{self, formatters},
    Color,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    AldousBroder,
    BinaryTree,
    Eller,
    GrowingTree,
    HuntAndKill,
    Kruskal,
    Prim,
    RecursiveBacktracking,
    RecursiveDivision,
    Sidewinder,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generates a maze
    Generate {
        #[command(subcommand)]
        output: OutputCommands,

        /// Maze generation algorithm
        #[arg(short = 'A', long, value_enum, default_value_t = Algorithm::RecursiveBacktracking)]
        algorithm: Algorithm,

        /// Grid height in a number of cells
        #[arg(short = 'H', long, default_value_t = 10)]
        height: usize,

        #[arg(short = 'W', long, default_value_t = 10)]
        /// Grid width in a number of cells
        width: usize,

        /// Bias to use for the "Binary Tree" algorithm
        #[arg(
            long,
            default_value_t = maze::Bias::NorthEast,
            require_equals = true,
            num_args = 0..=1,
            default_missing_value = "always",
            value_enum,
        )]
        bias: maze::Bias,

        /// Growing mwethod to use for the "Growing Tree" algorithm
        #[arg(
            long,
            default_value_t = maze::Method::Newest,
            require_equals = true,
            num_args = 0..=1,
            default_missing_value = "always",
            value_enum,
        )]
        growing_method: maze::Method,
    },
    /// Converts an input maze into given output
    Convert {},
}

#[derive(Debug, Subcommand)]
enum OutputCommands {
    /// Save to a text file with an ASCII representation of a maze
    Ascii {
        /// Output path
        #[arg(short = 'P', long)]
        output_path: String,
    },
    /// Save to a text file as a game map with colored walls for pseudo 3D games that use ray casting algorithm for modeling and rendering the map
    GameMap {
        /// Output path
        #[arg(short = 'P', long)]
        output_path: String,

        /// Distance between any two walls
        #[arg(long, default_value_t = 3)]
        span: usize,
    },
    /// Save to a text file as an ASCII game map for pseudo 3D games that use ray casting algorithm for modeling and rendering the map
    AsciiGameMap {
        /// Output path
        #[arg(short = 'P', long)]
        output_path: String,

        /// Distance between any two walls
        #[arg(long, default_value_t = 3)]
        span: usize,

        /// ASCII character for a passage
        #[arg(long, default_value_t = '.')]
        passage: char,

        /// ASCII character for a wall
        #[arg(long, default_value_t = '#')]
        wall: char,
    },
    /// Save to PNG or JPG file
    Image {
        /// Output path
        #[arg(short = 'P', long)]
        output_path: String,

        /// Wall size in pixels
        #[arg(long = "wall-size", default_value_t = 40)]
        wall_size: usize,

        /// Passage size in pixels
        #[arg(long = "passage-size", default_value_t = 40)]
        passage_size: usize,

        /// Size of the margin area that implies an empty space between an image borders and grid
        #[arg(long, default_value_t = 50)]
        margin: usize,

        /// Color of passages
        #[arg(long = "passage-color", default_value = "#ffffff", value_parser = hex_to_rgb)]
        passage_color: Color,

        /// Color of walls
        #[arg(long = "wall-color", default_value = "#000000", value_parser = hex_to_rgb)]
        wall_color: Color,
    },
}

fn main() -> Result<(), maze::MazeSaveError> {
    let args = Cli::parse();

    match args.command {
        Commands::Generate {
            output,
            algorithm,
            height,
            width,
            bias,
            growing_method,
        } => {
            let algorithm: Box<dyn maze::Algorithm> = match algorithm {
                Algorithm::AldousBroder => Box::new(maze::AldousBroder),
                Algorithm::BinaryTree => Box::new(maze::BinaryTree::new(bias)),
                Algorithm::Eller => Box::new(maze::Eller),
                Algorithm::GrowingTree => Box::new(maze::GrowingTree::new(growing_method)),
                Algorithm::HuntAndKill => Box::new(maze::HuntAndKill::new()),
                Algorithm::Kruskal => Box::new(maze::Kruskal),
                Algorithm::Prim => Box::new(maze::Prim::new()),
                Algorithm::RecursiveBacktracking => Box::new(maze::RecursiveBacktracking),
                Algorithm::RecursiveDivision => Box::new(maze::RecursiveDivision),
                Algorithm::Sidewinder => Box::new(maze::Sidewinder),
            };

            let maze = maze::OrthogonalMazeBuilder::new()
                .height(height)
                .width(width)
                .algorithm(algorithm)
                .build();

            match output {
                OutputCommands::Ascii { output_path } => {
                    maze.save(output_path.as_str(), formatters::Ascii)?;
                }
                OutputCommands::GameMap { output_path, span } => {
                    maze.save(output_path.as_str(), maze::GameMap::new().span(span))?;
                }
                OutputCommands::AsciiGameMap {
                    output_path,
                    span,
                    passage,
                    wall,
                } => {
                    maze.save(
                        output_path.as_str(),
                        maze::AsciiGameMap::new()
                            .span(span)
                            .passage(passage)
                            .wall(wall),
                    )?;
                }
                OutputCommands::Image {
                    output_path,
                    wall_size,
                    passage_size,
                    margin,
                    passage_color,
                    wall_color,
                } => {
                    maze.save(
                        output_path.as_str(),
                        maze::Image::new()
                            .wall(wall_size)
                            .passage(passage_size)
                            .margin(margin)
                            .background(passage_color)
                            .foreground(wall_color),
                    )?;
                }
            };

            Ok(())
        }
        Commands::Convert {} => todo!(),
    }
}

fn hex_to_rgb(s: &str) -> Result<Color, ParseHexError> {
    let s = if s.starts_with('#') { &s[1..] } else { s };

    if s.len() != 6 {
        return Err(ParseHexError::Length(s.to_string()));
    }

    let mut rgb = [0_u8; 3];
    rgb[0] = u8::from_str_radix(&s[..2], 16)?;
    rgb[1] = u8::from_str_radix(&s[2..4], 16)?;
    rgb[2] = u8::from_str_radix(&s[4..6], 16)?;
    Ok(Color::RGB(rgb[0], rgb[1], rgb[2]))
}

#[derive(Debug)]
enum ParseHexError {
    IntError(std::num::ParseIntError),
    Length(String),
}

impl std::error::Error for ParseHexError {}

impl std::fmt::Display for ParseHexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseHexError::Length(e) => write!(
                f,
                "Expected a 6 charactor color value in hex, but got: {:?}",
                e
            ),
            ParseHexError::IntError(ref e) => e.fmt(f),
        }
    }
}

impl From<std::num::ParseIntError> for ParseHexError {
    fn from(err: std::num::ParseIntError) -> ParseHexError {
        ParseHexError::IntError(err)
    }
}
