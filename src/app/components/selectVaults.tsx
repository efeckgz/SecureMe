import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, Trash2 } from "@geist-ui/icons";

const SelectVaults = ({ closeFunc }: { closeFunc: () => void }) => {
  const [vaults, setVaults] = useState([]);

  useEffect(() => {
    const getVaults = async () => {
      let vaults: VaultViewModel[] = await invoke("get_vaults");
      setVaults(vaults);
      console.log(vaults);
    };

    getVaults();
  }, []);

  return (
    <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10 bg-white/10">
      <div className="relative w-[700px] h-[500px] bg-black text-white rounded-lg">
        {/* Header Section */}
        <div className="flex justify-between items-center px-4 py-4 border-b border-white/10">
          <div className="flex items-center space-x-4">
            <button
              className="p-2 rounded hover:bg-white/10"
              onClick={() => closeFunc()}
              aria-label="Close vaults modal"
            >
              <X />
            </button>
            <h1 className="font-bold text-lg">Vault</h1>
          </div>
          <div className="flex space-x-4 text-lg font-bold">
            <span>Created</span>
            <span>Last accessed</span>
          </div>
        </div>

        {/* Modal Content */}
        <div className="overflow-y-auto h-[calc(100%-4rem)]">
          {/* <p className="text-center text-gray-400">Your vault content...</p> */}
          {vaults.map(({ name, path, isLocked }: VaultViewModel, key) => {
            return (
              <div
                className="flex justify-between items-center w-full px-4 py-4 border-b border-white/10 "
                key={key}
              >
                {name}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};

export default SelectVaults;
