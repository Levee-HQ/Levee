"use client";

interface WaterlineGaugeProps {
  totalCapital: number;
  lockedCapital: number;
  triggerProximity: number;
  height?: number;
  mini?: boolean;
}

export function WaterlineGauge({
  totalCapital,
  lockedCapital,
  triggerProximity,
  height = 200,
  mini = false,
}: WaterlineGaugeProps) {
  const waterLevel = totalCapital > 0 ? (totalCapital - lockedCapital) / totalCapital : 0;
  const floodStage = totalCapital > 0 ? lockedCapital / totalCapital : 0;
  const triggerLine = Math.min(triggerProximity, 1);

  const gaugeWidth = mini ? 32 : 48;
  const markerWidth = mini ? 8 : 16;
  const svgWidth = gaugeWidth + markerWidth * 2 + 40;

  const reduceMotion =
    typeof window !== "undefined" &&
    window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  return (
    <div
      className="relative inline-flex flex-col items-center"
      style={{ height }}
      role="img"
      aria-label={`Pool gauge: ${Math.round(waterLevel * 100)}% available, ${Math.round(floodStage * 100)}% locked, trigger at ${Math.round(triggerLine * 100)}%`}
    >
      <svg
        width={svgWidth}
        height={height}
        viewBox={`0 0 ${svgWidth} ${height}`}
        className="overflow-visible"
      >
        <defs>
          <linearGradient id="waterGradient" x1="0" y1="1" x2="0" y2="0">
            <stop offset="0%" stopColor="#0B2545" />
            <stop offset="100%" stopColor="#1B6CA8" />
          </linearGradient>
        </defs>

        {/* Gauge body */}
        <rect
          x={markerWidth + 20}
          y={0}
          width={gaugeWidth}
          height={height}
          rx={4}
          fill="#0B2545"
          stroke="#13507A"
          strokeWidth={1.5}
        />

        {/* Water level fill */}
        <rect
          x={markerWidth + 20 + 1}
          y={height - height * waterLevel}
          width={gaugeWidth - 2}
          height={height * waterLevel}
          rx={3}
          fill="url(#waterGradient)"
          opacity={0.8}
          style={
            reduceMotion
              ? undefined
              : {
                  transition: "y 0.6s ease-out, height 0.6s ease-out",
                }
          }
        />

        {/* Flood stage marker */}
        <line
          x1={markerWidth + 18}
          y1={height - height * floodStage}
          x2={markerWidth + 22 + gaugeWidth}
          y2={height - height * floodStage}
          stroke="#A3D5FF"
          strokeWidth={1.5}
          strokeDasharray="4 2"
        />
        {!mini && (
          <text
            x={markerWidth + gaugeWidth + 26}
            y={height - height * floodStage + 4}
            fill="#A3D5FF"
            fontSize={10}
            fontFamily="var(--font-tabular, monospace)"
          >
            LOCKED
          </text>
        )}

        {/* Trigger proximity line */}
        <line
          x1={markerWidth + 18}
          y1={height - height * triggerLine}
          x2={markerWidth + 22 + gaugeWidth}
          y2={height - height * triggerLine}
          stroke="#FF6B35"
          strokeWidth={2}
          style={
            reduceMotion
              ? undefined
              : {
                  transition: "y1 0.4s ease-out, y2 0.4s ease-out",
                }
          }
        />
        {!mini && (
          <text
            x={markerWidth + gaugeWidth + 26}
            y={height - height * triggerLine + 4}
            fill="#FF6B35"
            fontSize={10}
            fontWeight="bold"
            fontFamily="var(--font-tabular, monospace)"
          >
            TRIGGER
          </text>
        )}

        {/* Scale marks */}
        {[0, 0.25, 0.5, 0.75, 1].map((mark) => (
          <g key={mark}>
            <line
              x1={markerWidth + 14}
              y1={height - height * mark}
              x2={markerWidth + 20}
              y2={height - height * mark}
              stroke="#13507A"
              strokeWidth={1}
            />
            {!mini && (
              <text
                x={markerWidth + 10}
                y={height - height * mark + 3}
                fill="#13507A"
                fontSize={9}
                textAnchor="end"
                fontFamily="var(--font-tabular, monospace)"
              >
                {Math.round(mark * 100)}
              </text>
            )}
          </g>
        ))}
      </svg>
    </div>
  );
}
