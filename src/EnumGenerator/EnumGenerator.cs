using System.Text;

namespace RustEnumGenerator
{
    public partial class Form1 : Form
    {
        public Form1()
        {
            InitializeComponent();
        }

        struct Token
        {
            public string Text;
            public TokenType Type;
        }

        enum TokenType
        {
            Identifier,
            IntegerValue,
            Equals,
            Comma,
            LeftBrace,
            RightBrace,
            Enum
        }

        static bool IsNumber(string input)
        {
            foreach (char c in input)
            {

                if (char.IsDigit(c) == false)
                {
                    return false;
                }
            }
            return true;
        }

        static TokenType GetTokenType(string input)
        {
            if (IsNumber(input))
            {
                return TokenType.IntegerValue;
            }
            if (input == "=")
            {
                return TokenType.Equals;
            }
            if (input == ",")
            {
                return TokenType.Comma;
            }
            if (input == "{")
            {
                return TokenType.LeftBrace;
            }
            if (input == "}")
            {
                return TokenType.RightBrace;
            }
            if (input == "enum")
            {
                return TokenType.Enum;
            }
            return TokenType.Identifier;
        }

        static bool IsSkippableCharacter(char c)
        {
            if (c == ' ')
            {
                return true;
            }
            if (c == '\n')
            {
                return true;
            }
            if (c == '\r')
            {
                return true;
            }
            if (c == '\t')
            {
                return true;
            }
            return false;
        }

        static bool IsIdentifierCharacter(char c)
        {
            if (char.IsLetterOrDigit(c) == true)
            {
                return true;
            }
            if (c == '_')
            {
                return true;
            }
            return false;
        }

        static void FillIdentifier(string code, StringBuilder sb, ref int index)
        {
            int codeLength = code.Length;
            index += 1;
            while (index < codeLength)
            {
                char currentCharacter = code[index];
                if (IsIdentifierCharacter(currentCharacter) == false)
                {
                    break;
                }
                sb.Append(currentCharacter);
                index += 1;
            }
        }

        static void FillText(char firstCharacter, string code, StringBuilder sb, ref int index)
        {
            if (IsSkippableCharacter(firstCharacter) == true)
            {
                return;
            }
            if (firstCharacter == '=')
            {
                sb.Append(firstCharacter);
                return;
            }
            if (firstCharacter == ',')
            {
                sb.Append(firstCharacter);
                return;
            }
            if (firstCharacter == '{')
            {
                sb.Append(firstCharacter);
                return;
            }
            if (firstCharacter == '}')
            {
                sb.Append(firstCharacter);
                return;
            }
            sb.Append(firstCharacter);
            FillIdentifier(code, sb, ref index);
        }

        static Token? processCharacter(char firstCharacter, string code, ref int index)
        {
            StringBuilder sb = new StringBuilder();

            FillText(firstCharacter, code, sb, ref index);

            if (sb.Length == 0)
            {
                return null;
            }

            Token token = new Token();
            token.Text = sb.ToString();
            token.Type = GetTokenType(token.Text);
            return token;
        }

        static List<Token> Tokenize(string input)
        {
            List<Token> tokens = new List<Token>();

            int inputLength = input.Length;

            int indexBefore = -1;
            int currentIndex = 0;

            while (currentIndex < inputLength)
            {
                indexBefore = currentIndex;

                char currentChar = input[currentIndex];
                Token? token = processCharacter(currentChar, input, ref currentIndex);
                if (token.HasValue)
                {
                    tokens.Add(token.Value);
                }

                if (indexBefore == currentIndex)
                {
                    currentIndex += 1;
                }
            }

            return tokens;
        }

        void PrintTokens(List<Token> tokens)
        {
            foreach (Token token in tokens)
            {
                Console.WriteLine($"Token: '{token.Text}' Type: '{token.Type}'");
                txtDebug2.Text += $"Token: '{token.Text}' Type: '{token.Type}'\n";
            }
        }

        private void btnConvert_Click(object sender, EventArgs e)
        {
            lblError.Text = "";
            string input = txtInput.Text;

            List<Token> tokens = Tokenize(input);
            if (tokens.Count == 0)
            {
                lblError.Text = "Error: No tokens found.";
                return;
            }

            PrintTokens(tokens);

            ConvertData convertData = new ConvertData();
            convertData.TokenIndex = 0;
            convertData.Tokens = tokens;
            ConvertTokensToRustEnum(convertData);
        }

        class ConvertData
        {
            public int TokenIndex;
            public List<Token> Tokens;

            public Token? GetToken()
            {
                if (TokenIndex >= Tokens.Count) {
                    return null;
                }
                return Tokens[TokenIndex];
            }
            public void Increment() {
                TokenIndex += 1;
            }

        }

        bool CheckToken(Token? token, TokenType expectedType)
        {
            if (token == null) {
                lblError.Text = "Error: Unexpected end of tokens.";
                return false;
            }
            if (token.Value.Type != expectedType) {
                lblError.Text = $"Error: expecting {expectedType}.";
                return false;
            }
            return true;
        }

        void ConvertTokensToRustEnum(ConvertData convertData)
        {
            StringBuilder sb = new StringBuilder();

            Token? enumToken = convertData.GetToken();
            convertData.Increment();
            if (CheckToken(enumToken, TokenType.Enum) == false) {
                return;
            }

            Token? nameToken = convertData.GetToken();
            convertData.Increment();
            if (CheckToken(nameToken, TokenType.Identifier) == false) {
                return;
            }

            Token? leftBraceToken = convertData.GetToken();
            convertData.Increment();
            if (CheckToken(leftBraceToken, TokenType.LeftBrace) == false) {
                return;
            }

            List<Token> enumElements = new List<Token>();

            while (true)
            {
                Token? token = convertData.GetToken();
                convertData.Increment();
                if (token == null) {
                    lblError.Text = "Error: Unexpected end of tokens inside enum.";
                    return;
                }
                if (token.Value.Type == TokenType.RightBrace) {
                    break;
                }
                if (token.Value.Type == TokenType.Identifier) {
                    enumElements.Add(token.Value);
                }
            }

            sb.AppendLine($"pub struct {nameToken.Value.Text};");
            sb.AppendLine($"impl {nameToken.Value.Text} {{");
            int value = 0;
            foreach (Token element in enumElements)
            {
                sb.AppendLine($"    pub const {element.Text}: i32 = {value};");
                value += 1;
            }
            sb.AppendLine("}");

            string enumNameLower = nameToken.Value.Text.ToLower();
            sb.AppendLine($"pub fn {enumNameLower}_to_string({enumNameLower}: i32) -> &'static str {{");
            sb.AppendLine("    match " + enumNameLower + " {");
            foreach (Token element in enumElements)
            {
                sb.AppendLine($"        {nameToken.Value.Text}::{element.Text} => \"{element.Text}\",");
            }
            sb.AppendLine("        _ => \"Unknown\",");
            sb.AppendLine("    }");
            sb.AppendLine("}");

            txtOutput.Text = sb.ToString();
        }
    }
}
