
# Copied from:
# https://www.twilio.com/docs/iam/test-credentials?code-sample=code-create-a-message&code-language=curl&code-sdk-version=json#test-sms-messages
S = """
+15005550001 	This phone number is invalid. 	21212
+15005550007 	This phone number is not owned by your account or is not SMS-capable. 	21606
+15005550008 	This number has an SMS message queue that is full. 	21611
+15005550006 	This number passes all validation. 	No error
All Others 	This phone number is not owned by your account or is not SMS-capable. 	21606
To
Value 	Description 	Error Code
+15005550001 	This phone number is invalid. 	21211
+15005550002 	Twilio cannot route to this number. 	21612
+15005550003 	Your account doesn't have the international permissions necessary to SMS this number. 	21408
+15005550004 	This number is blacklisted for your account. 	21610
+15005550009 	This number is incapable of receiving SMS messages. 	21614
All Others 	Any other phone number is validated normally. 	Input-dependent
"""


m = False
i = 0
for l in S.strip().splitlines():
    i += 1
    t = l.strip().split()
    if t[0] == "To":
        m = True
        continue
    elif t[0] == "Value":
        continue
    t = l.split()
    p, c = t[0], t[-1]
    p = "VALID_NUMBER" if p == "All" else "\"{}\"".format(p)
    c = "None" if c in ["Input-dependent", "error"] else "Some({:d})".format(int(c))

    print("    #[test]")
    print("    fn test_{:d}() {{".format(i))
    if not m:
        print("        test_number({:s}, VALID_NUMBER, {!s});".format(p, c))
    else:
        print("        test_number(OUR_NUMBER, {:s}, {!s});".format(p, c))
    print("    }")
    print()

