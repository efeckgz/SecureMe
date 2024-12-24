import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, Trash2, Lock, Unlock } from "@geist-ui/icons";

import Button from "../components/common/button";
import CheckPassword from "./checkPassword";
import { useModal } from "../hooks/useModal";

interface VaultViewModel {
  name: string;
  path: string;
  isLocked: boolean;
}

interface VaultItemProps extends VaultViewModel {
  onDelete: () => void;
  onToggleLock: (path: string) => void;
}

const SelectVaults = () => {
  // Vaults retrieved from backend are stored here
  const [vaults, setVaults] = useState<VaultViewModel[]>([]);

  // The mode to show the check password modal
  // Set to "lock" when the vault is unlocked and "unlock" when it is locked.
  const [checkPassMode, setCheckPassMode] = useState("unlock");

  const { close } = useModal("vaults");
  const { isOpen: checkPassOpen } = useModal("checkPass");

  const [path, setPath] = useState("");

  const getVaults = async () => {
    const vaults: VaultViewModel[] = await invoke("get_vaults");
    console.log(vaults);
    setVaults(vaults);
  };

  useEffect(() => {
    getVaults();
  }, []);

  const deleteVault = async (path: string) => {
    // Issue a command to remove the vault of the given path
    await invoke("remove_vault", { path: path });

    // update the vaults state to trigger a re-render
    getVaults();
  };

  return (
    <>
      {checkPassOpen && <CheckPassword path={path} mode={checkPassMode} />}
      <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10 bg-white/10">
        <div className="relative w-[700px] h-[500px] bg-black text-white rounded-lg">
          {/* Header Section */}
          <div className="flex justify-between items-center px-4 py-4 border-b border-white/10">
            <div className="flex items-center space-x-4">
              <Button onClick={close}>
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
                  onDelete={async () => {
                    try {
                      await deleteVault(path);
                    } catch (e) {
                      console.log(e);
                    }
                  }}
                  onToggleLock={async (path: string) => {
                    setPath(path);

                    // Open in "unlock" mode if the vault is locked.
                    // Open in "lock" mode if the vault is unlocked.
                    setCheckPassMode(isLocked ? "unlock" : "lock");
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
  const subText = "font-thin text-sm text-white/50";

  const { open: openCheckPass } = useModal("checkPass");

  return (
    <div className="flex justify-between items-center w-full px-4 py-4 border-b border-white/10 ">
      <div className="flex flex-row items-center space-x-4">
        <Button onClick={() => onDelete()}>
          <Trash2 />
        </Button>
        <button
          className="flex flex-col"
          onClick={() => {
            onToggleLock(path);
            openCheckPass();
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
