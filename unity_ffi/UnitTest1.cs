using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Runtime.InteropServices;
using Support;
using System.Diagnostics;

namespace FFI_TESTS
{
    [TestClass]
    public class UnitTest1
    {
        [TestMethod]
        public void marshall_client_base()
        {
            Client client = new Client();
            Client.MClientBase cb = client.getBase();
            
            Assert.IsNotNull(cb.key);
            Assert.AreNotEqual(cb.key[0], 0);
        }

        
        [TestMethod]
        public void marshall_chat_frame()
        {
            Client client = new Client();

            // connect and login
            Assert.IsTrue((MBool)Client.client_connect(client, "127.0.0.1:9996"));
            Client.client_register(client);
            Assert.IsTrue((MBool)Client.client_login(client));

            //send something
            string text_s = "test";
            Chat.client_chat(client, text_s);

            System.Threading.Thread.Sleep(100);

            string text_r = client.getChat();

            Assert.AreEqual(text_s.Length, text_r.Length);
        }

        [TestMethod]
        public void timer_test()
        {
            Timer timer = new Timer(1);
            System.Threading.Thread.Sleep(1000);

            Assert.IsTrue(timer.tick());
        }
        
    }
}
