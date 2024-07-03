import { WagmiConfig, createConfig } from "wagmi";
import {
 
  getDefaultConfig,
} from "connectkit";
import { ReactNode, useState } from "react";
import { ToastContainer } from "react-toastify";
import Deposit from "./components/Deposit";
import { holesky } from "wagmi/chains";
import Modal from "./components/Modal";
import "react-toastify/dist/ReactToastify.css";
import {  AaProvider } from "./AaContext";
const { VITE_ALCHEMY_ID, VITE_WALLET_CONNECT_ID } = import.meta.env;

const config = createConfig(
  getDefaultConfig({
    alchemyId: VITE_ALCHEMY_ID,
    walletConnectProjectId: VITE_WALLET_CONNECT_ID,
    appName: "Compete Demo",
    chains: [holesky],
  })
);

function App() {


  return (

    <WagmiConfig config={config}>
          <AaProvider >

        <ToastContainer />
        
        <div className="app-container">
          <h2 className="title">Super Secret Project Demo</h2>
           <div className="card">
           <Deposit /> 
          </div>
          <p className="read-the-docs">This is for demo purposes only.</p>
        
        
        </div>
        <Footer />

        <ToastContainer />
      </AaProvider>
    </WagmiConfig>

  );
}



export default App;

function Footer() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [modalContent, setModalContent] = useState<{
    title: string;
    content: ReactNode;
  }>({ title: "", content: "" });

  const openModal = (title: string, content: ReactNode) => {
    setModalContent({ title, content });
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
  };

  return (
    <footer className="footer">
      <Modal
        isOpen={isModalOpen}
        onClose={closeModal}
        title={modalContent.title}
      >
        {modalContent.content}
      </Modal>
      <a href="https://www.risczero.com/news/bonsai-pay">About</a>
      <a
        href="https://github.com/risc0/demos/tree/main/bonsai-pay"
        className="footer-link"
      >
        Github
      </a>
      <a href="https://bonsai.xyz" className="footer-link">
        Bonsai
      </a>
      <button
        onClick={() =>
          openModal(
            "Terms of Service",
            <iframe
              className="tos-content"
              src="./BonsaiPayTermsofService2023.11.07.html"
              title="Terms of Service"
            />
          )
        }
        className="footer-button"
      >
        Terms of Service
      </button>
      <button
        onClick={() =>
          openModal(
            "Privacy Policy",
            <iframe
              className="privacy-content"
              src="./RISCZeroBonsaiWebsitePrivacyPolicy2023.11.07.html"
              title="Privacy Policy"
            />
          )
        }
        className="footer-button"
      >
        Privacy Policy
      </button>
    </footer>
  );
}
