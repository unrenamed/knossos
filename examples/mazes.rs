/// Extensive set of examples on how to use the lib to generate mazes and save them to files
use knossos::maze::*;

fn main() {
    let maze = OrthogonalMazeBuilder::new()
        .width(16)
        .height(18)
        .build()
        .unwrap();
    println!("\nRecursive backtracking");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(BinaryTree::new(Bias::NorthEast)))
        .build()
        .unwrap();
    println!("\nBinary Tree (NorthEast)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(BinaryTree::new(Bias::NorthWest)))
        .build()
        .unwrap();
    println!("\nBinary Tree (NorthWest)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(BinaryTree::new(Bias::SouthEast)))
        .build()
        .unwrap();
    println!("\nBinary Tree (SouthEast)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(BinaryTree::new(Bias::SouthWest)))
        .build()
        .unwrap();
    println!("\nBinary Tree (SouthWest)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .width(20)
        .algorithm(Box::new(Sidewinder))
        .build()
        .unwrap();
    println!("\nSidewinder");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .height(10)
        .width(15)
        .algorithm(Box::new(Kruskal))
        .build()
        .unwrap();

    println!("\nKruskal");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Newest)))
        .build()
        .unwrap();

    println!("\nGrowing tree (Newest)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Oldest)))
        .build()
        .unwrap();

    println!("\nGrowing tree (Oldest)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Random)))
        .build()
        .unwrap();

    println!("\nGrowing tree (Random)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Middle)))
        .build()
        .unwrap();

    println!("\nGrowing tree (Middle)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Newest50Random50)))
        .build()
        .unwrap();

    println!("\nGrowing tree (Newest50Random50)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Newest75Random25)))
        .build()
        .unwrap();

    println!("\nGrowing tree (Newest75Random25)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Newest25Random75)))
        .build()
        .unwrap();

    println!("\nGrowing tree (Newest25Random75)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(Prim::new()))
        .build()
        .unwrap();

    println!("\nPrim");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .height(10)
        .width(10)
        .algorithm(Box::new(HuntAndKill::new()))
        .build()
        .unwrap();

    println!("\nHunt & kill");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .height(25)
        .width(20)
        .algorithm(Box::new(AldousBroder))
        .build()
        .unwrap();

    println!("\nAldou-Broder");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(RecursiveDivision))
        .build()
        .unwrap();

    println!("\nRecursive Division");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .height(15)
        .width(15)
        .algorithm(Box::new(Eller))
        .build()
        .unwrap();

    println!("\nEller");
    println!("{}", &maze);

    maze.save("maze.txt", AsciiBroad).unwrap();
    maze.save(
        "maze_game_map.txt",
        GameMap::new().span(3).with_start_goal(),
    )
    .unwrap();
    maze.save("maze.png", Image::new().wall(15).passage(20))
        .unwrap();
}
