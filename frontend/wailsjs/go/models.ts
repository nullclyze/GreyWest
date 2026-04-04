export namespace app {
	
	export class PacketInfo {
	    length: number;
	    length_str: string;
	    protocol: string;
	    src_ip: string;
	    dst_ip: string;
	    identified: boolean;
	
	    static createFrom(source: any = {}) {
	        return new PacketInfo(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.length = source["length"];
	        this.length_str = source["length_str"];
	        this.protocol = source["protocol"];
	        this.src_ip = source["src_ip"];
	        this.dst_ip = source["dst_ip"];
	        this.identified = source["identified"];
	    }
	}

}

