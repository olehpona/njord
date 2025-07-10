import {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
  type ChartConfig,
} from "@/components/ui/chart";
import { CartesianGrid, Label, Line, LineChart, XAxis, YAxis } from "recharts";
import { Trash } from "lucide-react";
import {
  Table,
  TableBody,
  TableCell,
  TableHeader,
  TableRow,
} from "../ui/table";
import { Input } from "../ui/input";
import { Button } from "../ui/button";
import { usePlugContext } from "@/context/plug";
import { useEffect, useId, useState } from "react";
import { Label as BlockLabel } from "../ui/label";

const chartConfig = {
  value: {
    label: "Value",
    color: "orange",
  },
} satisfies ChartConfig;

interface Point {
  id: number;
  temp: number;
  value: number;
}

export default function PlugChart() {
  const { curve, setCurvePoints } = usePlugContext();

  const [points, setPoints] = useState<Point[]>(
    curve.map((el, index) => {
      const point: Point = {
        id: index,
        temp: el.temp,
        value: el.value,
      };
      return point;
    })
  );

  function addPoint() {
    setPoints([...points, { id: points.length, temp: 50, value: 50 }]);
  }

  function editPoint(id: number, temp: number, value: number) {
    let newCurve = [...points];
    newCurve[id] = {id: newCurve[id].id, temp, value };
    setPoints(newCurve);
  }

  function removePoint(id: number) {
    let newCurve = [...points];
    newCurve.splice(id, 1);
    setPoints(newCurve);
  }

  useEffect(() => {
    setCurvePoints(points);
  }, [points])

  useEffect(() => {}, [])

  const blockId = useId();

  return (
    <>
      <BlockLabel htmlFor={blockId} className="text-lg font-semibold">Cooling Curve</BlockLabel>
      <div id={blockId} className="w-full min-h-1/2 flex justify-center items-center">
        <ChartContainer className="w-full h-full" config={chartConfig}>
          <LineChart accessibilityLayer data={curve}>
            <CartesianGrid vertical={true} />
            <XAxis dataKey="temp" tickLine={false} axisLine={false}>
              <Label value="Temp °C" offset={0} position="insideBottom" />
            </XAxis>
            <YAxis dataKey="value" tickLine={false} axisLine={false}>
              <Label
                angle={-90}
                value="Value %"
                offset={0}
                position="insideLeft"
              />
            </YAxis>
            <ChartTooltip
              cursor={false}
              content={<ChartTooltipContent hideLabel />}
            />
            <Line
              dataKey="value"
              type="linear"
              stroke="red"
              strokeWidth={2}
              dot={false}
            />
          </LineChart>
        </ChartContainer>
      </div>
      <div className="w-full space-y-2">
        <Table>
          <TableHeader>
            <TableRow>
              <TableCell>Temp</TableCell>
              <TableCell>Value</TableCell>
            </TableRow>
          </TableHeader>
          <TableBody>
            {points.map((point, index) => (
              <TableRow key={point.id}>
                <TableCell>
                  <Input
                    type="number"
                    value={point.temp}
                    onChange={(e) =>
                      editPoint(
                        index,
                        e.target.valueAsNumber,
                        points[index].value
                      )
                    }
                  />
                </TableCell>
                <TableCell>
                  <Input
                    type="number"
                    min={0}
                    max={100}
                    onChange={(e) =>
                      editPoint(
                        index,
                        points[index].temp,
                        e.target.valueAsNumber
                      )
                    }
                    value={point.value}
                  />
                </TableCell>
                <TableCell>
                  <Button onClick={() => removePoint(index)} variant="ghost">
                    <Trash />
                  </Button>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
        <div className="w-full">
          <Button onClick={addPoint}>Add Point</Button>
        </div>
      </div>
    </>
  );
}
