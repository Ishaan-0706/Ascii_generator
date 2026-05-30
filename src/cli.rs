use clap::{Parser , Subcommand , Args}; //Parser does the parsing of the command line arguments, Subcommand is used to define subcommands, and Args is used to define the arguments for each subcommand.
use std::path::PathBuf; // path and pathbuf are used to handle file paths.

#[derive(Parser , Debug)]
#[command(
    name = "ascii_gen" ,
    version = "0.1.0" ,

)]
pub struct Cli{
    #[command(subcommand)] // this attribute tells clap that the field command is a subcommand, and it will look for the definition of the subcommands in the Command enum.
    pub command: Command,
}

//subcommands are used to define different functionalities of the program, and each subcommand can have its own arguments. In this case, we have two subcommands: Text and Image, which will be used to generate ASCII art from text and images respectively.



// derive is a macro that generates code for the specified traits. In this case, we are deriving the Debug trait for the Cli struct and the Command enum, which allows us to print the values of these types using the {:?} format specifier.
// why the subcommand is used: subcommands are used to define different functionalities of the program, and each subcommand can have its own arguments. In this case, we have two subcommands: Text and Image, which will be used to generate ASCII art from text and images respectively. This allows us to organize our code and make it easier to use the program from the command line. For example, we can run the program with the command "ascii_gen text --input "Hello World"" to generate ASCII art from the text "Hello World", or we can run "ascii_gen image --input path/to/image.png" to generate ASCII art from an image file.
#[derive(Subcommand , Debug)]
pub enum Command{
    Text(TextArgs),

    Image(ImageArgs),
}


#[derive(Args , Debug)]
pub struct TextArgs{
    //arg is the attribute that tells clap that the field text is an argument, and it will look for the definition of the argument in the TextArgs struct. The short and long attributes specify the short and long versions of the argument, which can be used in the command line to specify the value of the argument.
    #[arg(short , long)]
    // Text to render as a banner (printable ASCII only)
    pub input: String,

    // Output colour: red , blue , green , white, cyan , magenta , yellow
    #[arg(short , long , default_value = "white")]
    pub color: String,

    // Scale multilpirer : repeat the each pixel n times horizontally
    #[arg(short , long , default_value_t = 1)]
    pub scale: usize,

}


// Image subcommmand arguments

#[derive(Args , Debug)]
//this struct is for 
pub struct ImageArgs{
    // Path to source image 
    #[arg(short , long)]
    pub path: Pathbuf ,
    //pathbuf

    // Output width in characters (auto scaled heigth)
    #[arg(short , long , default_value_t = 100)]
    pub width: u32,

     #[arg(short , long , default_value_t = false)]
     pub detailed: bool,



     #[arg(short , long , default_value_t = 100)]
     pub invert: bool,

      #[arg(short , long)]
      pub output: Option<Pathbuf>,
}





}