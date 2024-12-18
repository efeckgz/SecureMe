import { useContext } from "react";
import { ModalContext } from "../contexts/ModalContext";

export const useModal = (modalId: string) => {
  const context = useContext(ModalContext);

  if (!context) {
    throw new Error("useModal must be used within a ModalProvider");
  }

  return {
    isOpen: context.modalStates[modalId] || false,
    open: () => context.openModal(modalId),
    close: () => context.closeModal(modalId),
  };
};
