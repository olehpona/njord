import CoolingHolder from "./cooling-holder";
import DeadAreas from "./dead_areas";
import PlugChart from "./plug-chart";
import SensorSelect from "./sensor-select";

export default function PlugSetup() {
  return (
    <>
      <SensorSelect />
      <PlugChart />
      <CoolingHolder/>
      <DeadAreas/>
    </>
  );
}
