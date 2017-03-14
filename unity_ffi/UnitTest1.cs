using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Runtime.InteropServices;
using Support;
using System.Diagnostics;
using System.Collections.Generic;

namespace FFI_TESTS
{
    [TestClass]
    public class UnitTest1
    {
        [TestMethod]
        public void marshall_client_base()
        {
            Client client = new Client();
            Client.MClientBase cb = client.GetBase();
            
            Assert.IsNotNull(cb.key);
            Assert.AreNotEqual(cb.key[0], 0);
        }

        [TestMethod]
        public void timer_test()
        {
            Timer timer = new Timer(1);
            System.Threading.Thread.Sleep(1000);

            Assert.IsTrue(timer.Tick());
        }

        [TestMethod]
        public void client_suite ()
        {
            Client client = new Client();

            // connect and login
            Assert.IsTrue((MBool)Client.client_connect(client, "127.0.0.1:9996"));
            Client.client_register(client);
            Assert.IsTrue((MBool)Client.client_login(client));

            //-- tests below --//
            marshall_chat_frame(client);
            poll_event(client);
        }

        void marshall_chat_frame(Client client)
        {
            //send something
            string text_s = "test";
            Chat.client_chat(client, text_s);

            System.Threading.Thread.Sleep(10);
            
            string msg = Chat.GetMsg(client, client.GetBase().id); //get our own msg

            Assert.AreEqual(text_s.Length, msg.Length);
        }
        
        void poll_event(Client client)
        {            
            Events ev = new Events(client);
            Assert.IsTrue(ev.has_event);
            Assert.AreNotEqual(ev.ev[0], 0);

            Assert.AreEqual(ev.GetEvent(), Events.Event.Player);
            Assert.AreNotEqual(ev.GetId()[0], 0);

            
            ev = new Events(client);
            Assert.IsTrue(ev.has_event);
            Assert.AreEqual(ev.GetEvent(), Events.Event.Player);
            
            ev = new Events(client);
            Assert.IsTrue(ev.has_event);
            Assert.AreEqual(ev.GetEvent(), Events.Event.Chat);
        }
    }
}
