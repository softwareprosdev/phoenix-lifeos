interface ScoreGaugeProps {
  score: number;
}

function getScoreColor(score: number): string {
  if (score >= 750) return "hsl(142, 72%, 45%)";
  if (score >= 700) return "hsl(196, 85%, 50%)";
  if (score >= 650) return "hsl(38, 92%, 50%)";
  return "hsl(0, 84%, 60%)";
}

function getScoreLabel(score: number): string {
  if (score >= 800) return "Exceptional";
  if (score >= 740) return "Very Good";
  if (score >= 670) return "Good";
  if (score >= 580) return "Fair";
  return "Poor";
}

export function ScoreGauge({ score }: ScoreGaugeProps) {
  const color = getScoreColor(score);
  const normalized = Math.min((score / 850) * 100, 100);
  const circumference = 2 * Math.PI * 54;
  const offset = circumference - (normalized / 100) * circumference;

  return (
    <div className="flex flex-col items-center">
      <div className="relative w-36 h-36">
        <svg className="w-36 h-36 -rotate-90" viewBox="0 0 120 120">
          <circle cx="60" cy="60" r="54" fill="none" stroke="hsl(220, 15%, 22%)" strokeWidth="8" />
          <circle
            cx="60"
            cy="60"
            r="54"
            fill="none"
            stroke={color}
            strokeWidth="8"
            strokeLinecap="round"
            strokeDasharray={circumference}
            strokeDashoffset={offset}
            style={{ transition: "stroke-dashoffset 1s ease-out" }}
          />
        </svg>
        <div className="absolute inset-0 flex flex-col items-center justify-center">
          <p className="text-3xl font-bold" style={{ color }}>{score}</p>
          <p className="text-[11px] font-medium" style={{ color }}>{getScoreLabel(score)}</p>
        </div>
      </div>
    </div>
  );
}
