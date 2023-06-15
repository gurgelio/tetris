import { SpeakerWaveIcon, SpeakerXMarkIcon } from "@heroicons/react/24/solid";
import { useVolume } from "../tetris/audio";

export function VolumeControls() {
  const { setVolume, isMuted, setIsMuted, effectiveVolume } = useVolume();

  const Icon = effectiveVolume() === 0 ? SpeakerXMarkIcon : SpeakerWaveIcon;

  return (
    <div className="flex flex-row items-center gap-3 my-3">
      <span
        className={`bg-neutral-600 p-2 rounded-md transition-colors ${
          effectiveVolume() === 0 ? "bg-opacity-50" : ""
        }`}
        onClick={() => setIsMuted(!isMuted)}
      >
        <Icon className="h-6 w-6" />
      </span>
      <input
        type="range"
        value={effectiveVolume() * 100}
        className="h-2 rounded-full bg-neutral-600 w-28"
        min="0"
        max="100"
        onChange={(e) => setVolume(parseInt(e.target.value) / 100)}
      />
    </div>
  );
}