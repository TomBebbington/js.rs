/// The compiler, which transforms Javascript expressions to LibJIT IR
pub mod compiler;
/// The executor, which runs the LibJIT IR by compiling it then running it
pub mod executor;