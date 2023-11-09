class Program
{
    static void Main(string[] args)
    {

        if (args.Length == 2)
        {
            string userIn = args[0];
            string dirOutput = args[1];
            if (File.Exists(userIn))
            {
                byte[] rawData = File.ReadAllBytes(userIn);
                byte[] decryptedFile = DecryptFile(rawData, userIn);
                string decryptedFilename = dirOutput + @"\" + Path.GetFileName(userIn);
                string? directoryToCreate = Path.GetDirectoryName(decryptedFilename);
                if (directoryToCreate != null)
                {
                    Directory.CreateDirectory(directoryToCreate);
                }
                File.WriteAllBytes(decryptedFilename, decryptedFile);
                Console.WriteLine("Single file decrypted and saved to: " + decryptedFilename);
            }
            else if (Directory.Exists(userIn))
            {
                DecryptFolder(userIn, dirOutput);
                Console.WriteLine("Directory processed: " + userIn);
            }
            else
            {
                Console.WriteLine("File or directory does not exist: " + userIn);
            }
        }
        else
        {
            string[] dirInputs = new string[] { @"www\img", @"www\audio", @"www\data" };
            string dirOutput = @"decrypted";
            foreach (string folder in dirInputs)
            {
                if (Directory.Exists(folder))
                {
                    DecryptFolder(folder, dirOutput);
                    Console.WriteLine("Directory processed: " + folder);
                }
                else
                {
                    Console.WriteLine("Directory does not exist: " + folder);
                }
            }
        }
        Console.WriteLine("All tasks finished. Press any key to exit.");
        Console.ReadKey();
    }

    static void DecryptFolder(string dirInput, string dirOutput)
    {
        string[] fileList = GetFiles(dirInput, "*.png|*.ogg|*.json", SearchOption.AllDirectories);
        foreach (string f in fileList)
        {
            byte[] rawData = File.ReadAllBytes(f);
            byte[] decryptedFile = DecryptFile(rawData, f);
            if (decryptedFile.Length == 1)
            {
                Console.WriteLine("Decoded signature doesn't match file header! Is the key wrong?");
                Console.WriteLine("Decryption aborted. File was NOT decrypted: " + f);
            }
            else
            {
                string decryptedFilename = dirOutput + @"\" + f;
                string? directoryToCreate = Path.GetDirectoryName(decryptedFilename);
                if (directoryToCreate != null)
                {
                    Directory.CreateDirectory(directoryToCreate);
                }
                File.WriteAllBytes(decryptedFilename, decryptedFile);
                Console.WriteLine("File decrypted and saved to: " + decryptedFilename);
            }
        }
    }
    static uint Mask(string inputString)
    {
        uint maskValue = 0;
        string decodedFilename = Path.GetFileNameWithoutExtension(inputString).ToUpper();
        foreach (char c in decodedFilename)
        {
            maskValue = (maskValue << 1) ^ c;
        }
        return maskValue;
    }
    static byte[] DecryptFile(byte[] inputData, string key)
    {
        uint maskValue = Mask(key);
        int signatureLength = Signature().Length;
        byte[] slicedInput = new byte[signatureLength];
        Buffer.BlockCopy(inputData,0,slicedInput,0,signatureLength);
        byte[] decodedChars = new byte[signatureLength];

        for (int i = 0; i < signatureLength; i++)
        {
            char c = Signature()[i];
            uint temp = (c ^ maskValue) % 0x100;
            decodedChars[i] = (byte)temp;
        }

        if (!Enumerable.SequenceEqual(slicedInput, decodedChars))
        {
            byte[] empty = { 0 };
            return empty;
        }

        byte[] remainingData = new byte[inputData.Length - signatureLength - 1];
        Buffer.BlockCopy(inputData, signatureLength + 1, remainingData, 0, remainingData.Length);

        byte zeroIndexValue = inputData[signatureLength];
        int decryptionLength = Convert.ToInt32(zeroIndexValue);

        if (zeroIndexValue == 0)
        {
            decryptionLength = remainingData.Length;
        }

        for (int i = 0; i < decryptionLength; i++)
        {
            byte temp = remainingData[i];
            uint decryptedByte = (remainingData[i] ^ maskValue) % 0x100;
            remainingData[i] = (byte)decryptedByte;
            maskValue = (maskValue << 1) ^ temp;
        }

        return remainingData;
    }
    static string[] GetFiles(string sourceFolder, string filters, SearchOption searchOption)
    {
        return filters.Split('|').SelectMany(filter => Directory.GetFiles(sourceFolder, filter, searchOption)).ToArray();
    }

    static string Signature()
    {
        string sig = "00000NEMLEI00000";
        return sig;
    }
}