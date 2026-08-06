"use client";

const TERMS = [
  { label: "1 day", ledgers: 17_280 },
  { label: "7 days", ledgers: 120_960 },
  { label: "14 days", ledgers: 241_920 },
  { label: "30 days", ledgers: 518_400 },
];

interface TermSelectorProps {
  value: number;
  onChange: (ledgers: number) => void;
}

export function TermSelector({ value, onChange }: TermSelectorProps) {
  return (
    <div className="flex gap-2">
      {TERMS.map((term) => (
        <button
          key={term.ledgers}
          onClick={() => onChange(term.ledgers)}
          className={`px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
            value === term.ledgers
              ? "bg-levee-surface text-levee-mark"
              : "bg-levee-deep border border-levee-mid/30 text-levee-light/60 hover:border-levee-surface/50"
          }`}
        >
          {term.label}
        </button>
      ))}
    </div>
  );
}
