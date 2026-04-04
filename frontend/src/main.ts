import { ApplyAutoSaver, ApplyPacketFilter, GetTotalPacketCountStr, RefreshAvailableInterfaces, ResetTotalPacketCount, StartPacketSniffing, StopPacketSniffing } from "../wailsjs/go/app/App.js";
import { EventsOn } from "../wailsjs/runtime/runtime.js";

/** Структура данных сетевого пакета */
interface NetworkPacket {
  length: number;
  length_str: string;
	protocol: string;
	src_ip: string;
	dst_ip: string;
  identified: boolean;
}

/** Структура данных сетевого интерфейса */
interface NetworkInterface {
  index: number;
  name: string;
  description: string;
  addresses: Array<string>;
}

/** Структура опций фильтра сетевых пакетов */
interface PacketFilter {
  protocol: string;
  src_ip: string;
  dst_ip: string;
}

/** Структура опций авто сохранения */
interface AutoSaver {
  directory: string;
  filename: string;
}

/** Последний выбранный индекс сетевого интерфейса */
let lastSelectedInterfaceIndex = -1;

/** Счетчик отображаемых пакетов */
let packetCounter = 0;

/** Функция сброса данных мониторинга */
const resetMonitoring = async () => {
  await ResetTotalPacketCount();
  (document.getElementById("total-packet-count") as HTMLSpanElement).innerText = "0";
  (document.getElementById("packet-list") as HTMLDivElement).innerHTML = "";
  packetCounter = 0;
}

/** Метод обновления интерейсов и их функций */
const updateInterfaces = () => {
  const startPage = document.getElementById("start-page") as HTMLDivElement;
  const monitoringPage = document.getElementById("monitoring-page") as HTMLDivElement;

  const interfaceElements = document.querySelectorAll<HTMLDivElement>("#interface-list .interface");

  interfaceElements.forEach(el => el.addEventListener("click", async () => {
    const indexStr = el.getAttribute("interface-idx");
    if (!indexStr) return;
    const index = parseInt(indexStr);

    if (index === lastSelectedInterfaceIndex) {
      startPage.style.display = "flex";
      monitoringPage.style.display = "none";

      el.classList.remove("selected");
      
      lastSelectedInterfaceIndex = -1;

      await StopPacketSniffing();
      await resetMonitoring();
    } else {
      startPage.style.display = "none";
      monitoringPage.style.display = "flex";

      interfaceElements.forEach(e => e.classList.remove("selected"));
      el.classList.add("selected");

      lastSelectedInterfaceIndex = index;

      await StopPacketSniffing();
      await resetMonitoring();
      await StartPacketSniffing(index);
    }
  }));
}

/** Функция применения фильтра */
const applyFilter = async () => {
  const packetList = document.getElementById("packet-list") as HTMLDivElement;
  packetList.innerHTML = "";
  packetCounter = 0;

  const protocol = (document.getElementById("filter-protocol") as HTMLInputElement).value;
  const srcIp = (document.getElementById("filter-src-ip") as HTMLInputElement).value;
  const dstIp = (document.getElementById("filter-dst-ip") as HTMLInputElement).value;

  let filter: PacketFilter = {
    protocol: protocol,
    src_ip: srcIp,
    dst_ip: dstIp
  };

  localStorage.setItem("packet-filter", JSON.stringify(filter));

  await ApplyPacketFilter(protocol, srcIp, dstIp);
}

/** Функция применения авто сохранения */
const applySaver = async () => {
  const directory = (document.getElementById("saver-directory") as HTMLInputElement).value;
  const filename = (document.getElementById("saver-filename") as HTMLInputElement).value;

  let saver: AutoSaver = {
    directory: directory,
    filename: filename,
  };

  localStorage.setItem("auto-saver", JSON.stringify(saver));

  await ApplyAutoSaver(directory, filename);
}

/** Функция загрузки опций фильтра */
const loadFilterOptions = async () => {
  const data = localStorage.getItem("packet-filter");

  if (data) {
    const packetFilter: PacketFilter = JSON.parse(data);

    (document.getElementById("filter-protocol") as HTMLInputElement).value = packetFilter.protocol;
    (document.getElementById("filter-src-ip") as HTMLInputElement).value = packetFilter.src_ip;
    (document.getElementById("filter-dst-ip") as HTMLInputElement).value = packetFilter.dst_ip;
  }

  await applyFilter();
}

/** Функция загрузки опций авто сохранения */
const loadSaverOptions = async () => {
  const data = localStorage.getItem("auto-saver");

  if (data) {
    const autoSaver: AutoSaver = JSON.parse(data);

    (document.getElementById("saver-directory") as HTMLInputElement).value = autoSaver.directory;
    (document.getElementById("saver-filename") as HTMLInputElement).value = autoSaver.filename;
  }

  await applySaver();
}

/** Обработчик события "refresh-interfaces" */
const refreshInterfacesHandler = (interfaces: Array<NetworkInterface>) => {
  const interfaceList = document.getElementById("interface-list") as HTMLDivElement;

  interfaceList.innerHTML = "";

  for (const iface of interfaces) {  
    const ifaceEl = document.createElement("div");
    ifaceEl.className = "interface";
    ifaceEl.setAttribute("interface-idx", iface.index.toString());

    const ifaceName = document.createElement("div");
    ifaceName.className = "name";
    ifaceName.innerText = iface.description;

    const ifaceAddresses = document.createElement("div");
    ifaceAddresses.className = "addresses";

    if (iface.addresses.length > 0) {
      for (const addr of iface.addresses) {
        const ifaceAddress = document.createElement("p");
        ifaceAddress.innerText = addr;
        ifaceAddresses.appendChild(ifaceAddress);
      }
    } else {
      const ifaceNoAddr = document.createElement("p");
      ifaceNoAddr.innerText = "Нету адресов";
      ifaceAddresses.appendChild(ifaceNoAddr);
    }

    ifaceEl.appendChild(ifaceName);
    ifaceEl.appendChild(ifaceAddresses);
    
    interfaceList.appendChild(ifaceEl);
  }

  updateInterfaces();
}

/** Обработчик события "network-packet" */
const networkPacketHandler = async (packet: NetworkPacket) => {
  const totalPacketCount = document.getElementById("total-packet-count") as HTMLSpanElement;
  const packetList = document.getElementById("packet-list") as HTMLDivElement;

  if (packetCounter >= 100) {
    packetList.firstChild?.remove();
    packetCounter = 99;
  }

  const row = document.createElement("div");
  row.className = "row";

  const protocol = document.createElement("span");
  protocol.className = "col-protocol";
  protocol.innerText = packet.protocol;

  const source = document.createElement("span");
  source.className = "col-source";
  source.innerText = packet.src_ip;

  const destination = document.createElement("span");
  destination.className = "col-destination";
  destination.innerText = packet.dst_ip;

  const length = document.createElement("span");
  length.className = "col-length";
  length.innerText = packet.length_str;

  row.appendChild(protocol);
  row.appendChild(source);
  row.appendChild(destination);
  row.appendChild(length);

  packetList.appendChild(row);
  
  packetCounter++;
  totalPacketCount.innerText = await GetTotalPacketCountStr();

  if ((document.getElementById("packet-auto-scroll") as HTMLInputElement).checked) {
    packetList.scrollTo({ 
      top: packetList.scrollHeight, 
      behavior: "smooth"
    });
  }
}

/** Функция инициализации */
export const setup = async () => {
  EventsOn("refresh-interfaces", refreshInterfacesHandler);
  EventsOn("network-packet", networkPacketHandler);

  const monitoringSettings = document.getElementById("monitoring-settings") as HTMLDivElement;

  document.getElementById("open-monitoring-settings")?.addEventListener("click", () => monitoringSettings.style.display = "flex");
  document.getElementById("close-monitoring-settings")?.addEventListener("click", () => monitoringSettings.style.display = "none");

  document.getElementById("apply-filter")?.addEventListener("click", applyFilter);
  document.getElementById("apply-saver")?.addEventListener("click", applySaver);

  await loadFilterOptions();
  await loadSaverOptions();

  await RefreshAvailableInterfaces();
}

document.addEventListener("DOMContentLoaded", setup);