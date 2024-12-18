"use client";

import { createContext, useState } from "react";

type ModalContextType = {
  modalStates: Record<string, boolean>;
  openModal: (modalId: string) => void;
  closeModal: (modalId: string) => void;
  isAnyModalOpen: boolean;
};

export const ModalContext = createContext<ModalContextType | undefined>(
  undefined
);

export const ModalProvider = ({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) => {
  const [modalStates, setModalStates] = useState<Record<string, boolean>>({
    vaults: false,
    enterCredentials: false,
    checkPass: false,
  });

  const openModal = (modalId: string) => {
    console.log("opening", modalId);
    setModalStates((prev) => ({
      ...prev,
      [modalId]: true,
    }));
  };

  const closeModal = (modalId: string) => {
    console.log("closing", modalId);
    setModalStates((prev) => ({
      ...prev,
      [modalId]: false,
    }));
  };

  const isAnyModalOpen = Object.values(modalStates).some((state) => state);

  return (
    <ModalContext.Provider
      value={{
        modalStates,
        openModal,
        closeModal,
        isAnyModalOpen,
      }}
    >
      {children}
    </ModalContext.Provider>
  );
};
