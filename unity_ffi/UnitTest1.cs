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
            IntPtr client = FFI.default_client();

            FFI.drop_client(client);
            MBool is_null = FFI.drop_client(client);

            //Assert.IsTrue(is_null);
            //Assert.AreEqual(client, IntPtr.Zero);
        }

        [TestMethod]
        public void marshall_client_base()
        {
            IntPtr client = FFI.default_client();
            MClientBase cb = new MClientBase();

            FFI.get_client_base(client, ref cb);
            Assert.IsNotNull(cb.key);
            Assert.AreNotEqual(cb.key[0], 0);
        }

        [TestMethod]
        public void marshall_chat_frame()
        {
            IntPtr client = FFI.default_client();

            // connect and login
            {
                MBool r = FFI.client_connect(client, "127.0.0.1:9996");
                Assert.IsTrue(r);
            }
            FFI.client_register(client);
            {
                MBool r = FFI.client_login(client);
                Assert.IsTrue(r);
            }

            //send something
            string text = "test";
            FFI.client_chat(client, text);

            System.Threading.Thread.Sleep(100);

            MChatFrame chat = new MChatFrame();
            UInt16 chat_len = FFI.get_client_chat(client, ref chat);

            Assert.AreEqual(chat_len, text.Length);
            Assert.IsNotNull(chat.msg);
            Assert.AreNotEqual(chat.msg[0], 0);
            Assert.AreEqual(chat.get_msg(chat_len), text);
        }

        [TestMethod]
        public void stream_hup()
        {
            IntPtr client = FFI.default_client();

            // connect and login
            {
                MBool r = FFI.client_connect(client, "127.0.0.1:9996");
                Assert.IsTrue(r);
            }
            FFI.client_register(client);
            {
                MBool r = FFI.client_login(client);
                Assert.IsTrue(r);
            }

            {
                MBool r = FFI.client_disconnect(client);
                Assert.IsTrue(r);
            }

            System.Threading.Thread.Sleep(1000);
            float ping = FFI.get_client_ping(client);
            Assert.IsTrue(ping > 0);
            Assert.IsFalse((MBool)FFI.is_client_connected(client));
        }
    }
}
