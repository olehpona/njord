import { usePlugContext } from "@/context/plug";
import { Label } from "../ui/label";
import {
  Table,
  TableBody,
  TableCell,
  TableHeader,
  TableRow,
} from "../ui/table";
import { Input } from "../ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";
import { Button } from "../ui/button";
import { Trash } from "lucide-react";

export default function DeadAreas() {
  const { dead_areas, setDeadAreas } = usePlugContext();

  function addDeadArea() {
    setDeadAreas([
      ...dead_areas,
      { min_value: 0, max_value: 0, variant: "Min" },
    ]);
  }

  function editDeadAreas(
    index: number,
    min_value: number,
    max_value: number,
    variant: string
  ) {
    let newDeadAreas = [...dead_areas];
    newDeadAreas[index] = { min_value, max_value, variant };
    setDeadAreas(newDeadAreas);
  }

  function removeDeadArea(index: number) {
    let newDeadAreas = [...dead_areas];
    newDeadAreas.splice(index, 1);
    setDeadAreas(newDeadAreas);
  }

  return (
    <div className="w-full">
      <Label className="font-semibold text-lg">Dead Areas</Label>
      <div className="w-full space-y-2">
        <Table>
          <TableHeader>
            <TableRow>
              <TableCell>Min value</TableCell>
              <TableCell>Max value</TableCell>
              <TableCell>Variant</TableCell>
            </TableRow>
          </TableHeader>
          <TableBody>
            {dead_areas.map((dead_area, index) => (
              <TableRow key={index}>
                <TableCell>
                  <Input
                    type="number"
                    min={0}
                    max={100}
                    value={dead_area.min_value}
                    onChange={(e) =>
                      editDeadAreas(
                        index,
                        e.target.valueAsNumber,
                        dead_areas[index].max_value,
                        dead_areas[index].variant
                      )
                    }
                  />
                </TableCell>
                <TableCell>
                  <Input
                    type="number"
                    min={0}
                    max={100}
                    value={dead_area.max_value}
                    onChange={(e) =>
                      editDeadAreas(
                        index,
                        dead_areas[index].min_value,
                        e.target.valueAsNumber,
                        dead_areas[index].variant
                      )
                    }
                  />
                </TableCell>
                <TableCell>
                  <Select
                    value={dead_area.variant}
                    onValueChange={(value) =>
                      editDeadAreas(
                        index,
                        dead_areas[index].min_value,
                        dead_areas[index].max_value,
                        value
                      )
                    }
                  >
                    <SelectTrigger className="w-20">
                      <SelectValue placeholder="Select Variant"></SelectValue>
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="Min">Min</SelectItem>
                      <SelectItem value="Max">Max</SelectItem>
                      <SelectItem value="Center">Center</SelectItem>
                    </SelectContent>
                  </Select>
                </TableCell>
                <TableCell>
                  <Button onClick={() => removeDeadArea(index)} variant="ghost">
                    <Trash />
                  </Button>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
        <div className="w-full">
          <Button onClick={addDeadArea}>Add Area</Button>
        </div>
      </div>
    </div>
  );
}
