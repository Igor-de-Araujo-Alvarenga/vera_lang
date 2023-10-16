using System.Text.RegularExpressions;

namespace Lexer
{
   public class Lex
    {
        private readonly List<Token> tokens;
        private int currentPosition;
        private string input;

        public Lex(string input)
        {
            this.input = input;
            tokens = new List<Token>();
            currentPosition = 0;
        }

        public List<Token> Tokenize()
        {
            var tokenDefinitions = Initialized();

            var tokenRegex = string.Join("|", tokenDefinitions.Select(td => $"(?<{td.TokenType}>{td.Pattern})"));

            var regex = new Regex(tokenRegex);

            while (currentPosition < input.Length)
            {
                var match = regex.Match(input, currentPosition);

                if (!match.Success)
                {
                    throw new Exception($"Invalid token at position {currentPosition}");
                }

                var tokenType = tokenDefinitions.First(td => match.Groups[td.TokenType].Success).TokenType;
                var lexeme = match.Value;
                tokens.Add(new Token { TokenType = tokenType, Lexeme = lexeme });
                currentPosition = match.Index + match.Length;
            }

            return tokens;
        }

        public (string TokenType, string Pattern)[] Initialized()
        {
            return new (string TokenType, string Pattern)[]
            {
            ("PROGRAM", "program"),
            ("MAIN", "main"),
            ("LEFT_BRACE", "{"),
            ("RIGHT_BRACE", "}"),
            ("IF", "if"),
            ("ELSE", "else"),
            ("LEFT_PAREN", "\\("),
            ("RIGHT_PAREN", "\\)"),
            ("COMMA", ","),
            ("SEMICOLON", ";"),
            ("ASSIGNMENT", "="),
            ("RETURN", "return"),
            ("BREAK", "break"),
            ("CONTINUE", "continue"),
            ("INT", "int"),
            ("FLOAT", "float"),
            ("STRING", "string"),
            ("BOOL", "bool"),
            ("IDENTIFIER", "[a-zA-Z_][a-zA-Z0-9_]*"),
            ("NUMBER", @"\d+(\.\d+)?"),
            ("STRING_LITERAL", "\"[^\"]*\""),
            ("TRUE", "true"),
            ("FALSE", "false"),
            ("PLUS", "\\+"),
            ("MINUS", "-"),
            ("MULTIPLY", "\\*"),
            ("DIVIDE", "/")
            };
        }
    }
        
}