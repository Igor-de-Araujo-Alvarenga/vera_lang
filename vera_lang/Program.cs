using Lexer;

string filePath = @"C:/repo_sharp/vera_lang/main.vera";

if (File.Exists(filePath))
{
    string input = File.ReadAllText(filePath);
    var lexer = new Lex(input);
    var tokens = lexer.Tokenize();

    foreach (var token in tokens)
    {
        Console.WriteLine($"{token.TokenType}: {token.Lexeme}");
    }
}
else
{
    Console.WriteLine("File not found.");
}