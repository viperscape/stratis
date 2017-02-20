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

        [TestMethod]
        public void marshall_chat_frame()
        {
            IntPtr client = FFI.new_client();

            // connect and login
            {
                byte r = FFI.client_connect(client, "127.0.0.1:9996");
                Assert.AreEqual(r, 1);
            }
            FFI.client_register(client);
            {
                byte r = FFI.client_login(client);
                Assert.AreEqual(r, 1);
            }

            //send something
            string text = "test";
            FFI.client_chat(client,text);

            System.Threading.Thread.Sleep(100);

            MChatFrame chat = new MChatFrame();
            UInt16 chat_len = FFI.get_client_chat(client, ref chat);

            Assert.AreEqual(chat_len, text.Length);
            Assert.IsNotNull(chat.msg);
            Assert.AreNotEqual(chat.msg[0],0);
            Assert.AreEqual(chat.get_msg(chat_len), text);
        }
    }
}
