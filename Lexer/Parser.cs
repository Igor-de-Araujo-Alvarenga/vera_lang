using Lexer;
using System;
using System.Collections.Generic;
using System.Linq;

public class Parser
{
    private List<Token> tokens;
    private int currentPosition;

    public Parser(List<Token> tokens)
    {
        this.tokens = tokens;
        currentPosition = 0;
    }

    public Node ParseProgram()
    {
        return ParseBlock();
    }

    public Node ParseBlock()
    {
        Match("MAIN");
        Match("LEFT_BRACE");
        var statements = new List<Node>();

        while (!Peek("RIGHT_BRACE"))
        {
            var stmt = ParseStatement();
            statements.Add(stmt);
        }

        return new BlockNode(statements);
    }

    public Node ParseStatement()
    {
        if (Peek("IF"))
            return ParseIfStatement();
        if (Peek("RETURN"))
            return ParseReturnStatement();
        if (Peek("BREAK"))
            return ParseBreakStatement();
        if (Peek("CONTINUE"))
            return ParseContinueStatement();

        return ParseAssignment();
    }

    public Node ParseIfStatement()
    {
        Match("IF");
        var condition = ParseCondition();
        Match("LEFT_BRACE");
        var ifBlock = ParseBlock();
        Match("RIGHT_BRACE");

        if (Peek("ELSE"))
        {
            Match("ELSE");
            Match("LEFT_BRACE");
            var elseBlock = ParseBlock();
            Match("RIGHT_BRACE");
            return new IfStatementNode(condition, ifBlock, elseBlock);
        }

        return new IfStatementNode(condition, ifBlock);
    }

    public Node ParseReturnStatement()
    {
        Match("RETURN");
        var expression = ParseExpression();
        Match("SEMICOLON");
        return new ReturnStatementNode(expression);
    }

    public Node ParseBreakStatement()
    {
        Match("BREAK");
        Match("SEMICOLON");
        return new BreakStatementNode();
    }

    public Node ParseContinueStatement()
    {
        Match("CONTINUE");
        Match("SEMICOLON");
        return new ContinueStatementNode();
    }

    public Node ParseAssignment()
    {
        var identifier = Match("IDENTIFIER").Lexeme;
        Match("ASSIGNMENT");
        var expression = ParseExpression();
        Match("SEMICOLON");
        return new AssignmentNode(identifier, expression);
    }

    public Node ParseCondition()
    {
        return ParseExpression();
    }

    public Node ParseExpression()
    {
        // Implement expression parsing here
        throw new NotImplementedException("Expression parsing not implemented.");
    }

    // Implement the remaining parsing methods

    private Token Match(string expectedToken)
    {
        if (currentPosition >= tokens.Count)
        {
            throw new Exception($"Unexpected end of input. Expected: {expectedToken}");
        }

        var currentToken = tokens[currentPosition];

        if (currentToken.TokenType == expectedToken)
        {
            currentPosition++;
            return currentToken;
        }
        else
        {
            throw new Exception($"Syntax error. Expected: {expectedToken}, but got: {currentToken.TokenType}");
        }
    }

    private bool Peek(string expectedToken)
    {
        return currentPosition < tokens.Count && tokens[currentPosition].TokenType == expectedToken;
    }

    // Implement AST node classes based on your language's structure
}
public class IfStatementNode : Node
{
    public Node Condition { get; }
    public Node IfBlock { get; }
    public Node ElseBlock { get; }

    public IfStatementNode(Node condition, Node ifBlock, Node elseBlock = null)
    {
        Condition = condition;
        IfBlock = ifBlock;
        ElseBlock = elseBlock;
    }
}