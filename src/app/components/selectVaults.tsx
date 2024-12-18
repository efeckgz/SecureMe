import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, Trash2, Lock, Unlock, Check } from "@geist-ui/icons";

import Button from "../components/common/button";
import CheckPassword from "./checkPassword";

const SelectVaults = ({ closeFunc }: { closeFunc: () => void }) => {
  // Vaults retrieved from backend are stored here
  const [vaults, setVaults] = useState([]);

  const [unlockModal, setUnlockModal] = useState<{
    shown: boolean;
    path: string;
  }>({
    shown: false,
    path: "",
  });

  useEffect(() => {
    const getVaults = async () => {
      let vaults: VaultViewModel[] = await invoke("get_vaults");
      setVaults(vaults);
    };

    getVaults();
  }, []);

  const deleteVault = async (path: string) => {
    // Issue a command to remove the vault of the given path
    await invoke("remove_vault", { path: path });

    // update the vaults state to trigger a re-render
    let vaults: VaultViewModel[] = await invoke("get_vaults");
    setVaults(vaults);
  };

  return (
    <>
      {unlockModal.shown && (
        <CheckPassword
          closeFunc={() => setUnlockModal({ shown: false, path: "" })}
          path={unlockModal.path}
        />
      )}
      <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10 bg-white/10">
        <div className="relative w-[700px] h-[500px] bg-black text-white rounded-lg">
          {/* Header Section */}
          <div className="flex justify-between items-center px-4 py-4 border-b border-white/10">
            <div className="flex items-center space-x-4">
              <Button onClick={() => closeFunc()}>
                <X />
              </Button>
              <h1 className="font-bold text-lg">Vault</h1>
            </div>
            <div className="flex space-x-4 text-lg font-bold">
              <span>Created</span>
              <span>Last accessed</span>
            </div>
          </div>
          {/* Modal Content */}
          <div className="overflow-y-auto h-[calc(97%-4rem)]">
            {/* <p className="text-center text-gray-400">Your vault content...</p> */}
            {vaults.map(({ name, path, isLocked }: VaultViewModel, key) => {
              return (
                <VaultItem
                  onDelete={async () => await deleteVault(path)}
                  onToggleLock={async (path: string) => {
                    setUnlockModal({ shown: true, path: path });
                  }}
                  name={name}
                  path={path}
                  isLocked={isLocked}
                  key={key}
                />
              );
            })}
          </div>
        </div>
      </div>
    </>
  );
};

const VaultItem = ({
  name,
  path,
  isLocked,
  onDelete,
  onToggleLock,
}: VaultItemProps) => {
  // const [vaultLocked, setVaultLocked] = useState(isLocked);

  const subText = "font-thin text-sm text-white/50";

  return (
    <div className="flex justify-between items-center w-full px-4 py-4 border-b border-white/10 ">
      <div className="flex flex-row items-center space-x-4">
        <Button onClick={() => onDelete()}>
          <Trash2 />
        </Button>
        <button
          className="flex flex-col"
          onClick={() => {
            // setVaultLocked(!vaultLocked);
            onToggleLock(path);
          }}
        >
          <h1 className="font-bold text-lg">{name}</h1>
          <h2 className={subText}>{path}</h2>
        </button>
      </div>
      {isLocked ? <Lock /> : <Unlock />}
      <h2 className={subText}>Creation date</h2>
      <h2 className={subText}>Last accessed</h2>
    </div>
  );
};

export default SelectVaults;
