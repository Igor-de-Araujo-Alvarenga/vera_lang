public class ReturnStatementNode : Node
{
    private Node expression;

    public ReturnStatementNode(Node expression)
    {
        this.expression = expression;
    }
}