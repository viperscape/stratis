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
        /*[TestMethod]
        public void smoke()
        {
            IntPtr client = FFI.new_client();
            FFI.drop_client(client);
            Assert.IsNull(client);
        }*/

        [TestMethod]
        public void marshall_client_base()
        {
            IntPtr client = FFI.new_client();
            MClientBase cb = new MClientBase();

            FFI.get_client_base(client, ref cb);
            Assert.IsNotNull(cb.key);
            Assert.AreNotEqual(cb.key[0], 0);
        }
    }
}
