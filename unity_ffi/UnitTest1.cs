using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Runtime.InteropServices;
using Assets;
using System.Diagnostics;

namespace FFI_TESTS
{
    [TestClass]
    public class UnitTest1
    {
        [TestMethod]
        public void smoke()
        {
            IntPtr client = FFI.new_client();
            FFI.drop_client(client);
            Assert.IsNull(client);
        }

        [TestMethod]
        public void marshall_id()
        {
            IntPtr client = FFI.new_client();
            byte[] id = new byte[16];
            id = FFI.get_client_id(client);
            Assert.AreNotEqual(id, new byte[16]);
        }
    }
}
