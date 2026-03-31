import { memo, type CSSProperties } from "react";
import type { MixCard } from "../data/library";

export interface MixGridProps {
  mixes: MixCard[];
}

export const MixGrid = memo(function MixGrid({ mixes }: MixGridProps) {
  return (
    <div className="mixes">
      {mixes.map((mix) => (
        <article className="mix" key={mix.title}>
          <div className="mix-art" style={mix.tone ? ({ "--art-tone": mix.tone } as CSSProperties) : undefined} />
          <span className="title-ellipsis">{mix.title}</span>
          <span className="sub-ellipsis">{mix.meta}</span>
        </article>
      ))}
    </div>
  );
});
