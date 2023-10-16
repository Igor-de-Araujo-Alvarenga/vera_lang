public class AssignmentNode : Node
{
    private string identifier;
    private Node expression;

    public AssignmentNode(string identifier, Node expression)
    {
        this.identifier = identifier;
        this.expression = expression;
    }
}