use itertools::Itertools;

fn main() {
    let vpc_ieee = vec!(
        "Modular Virtual Preparation method of production systems using a Digital Twin architecture",
        "Virtual Commissioning Simulation as OpenAI Gym - A Reinforcement Learning Environment for Control Systems",
        "Hybrid Commissioning of Production Plants",
        "Mechatronic Swarm and its Virtual Commissioning",
        "Supporting the Design, Commissioning and Supervision of Smart Factory Components through their Digital Twin",
        "Virtual Testing in Automated Driving Systems Certification. A Longitudinal Dynamics Validation Example",
        "A Virtual Commissioning Selection Approach for Machine Automation",
        "Automatic Generation of Charging Point's Digital Twin for Virtual Commissioning of Their Automation Systems",
        "Manufacturing machine virtual commissioning: Automated validation of the control software",
        "Fault injection in Digital Twin as a means to test the response to process faults at virtual commissioning",
        "Concept For An Interoperable Behavior Library For Virtual Commissioning Using PLCopenXML",
        "Validation of dynamic interoperability and virtual commissioning of production equipment in early development stages",
        "Virtual Commissioning Approach based on the Discrete Element Method",
        "Advanced tools for the control engineer in Industry 4.0",
        "Integrated virtual commissioning of a ROS2-based collaborative and intelligent automation system",
        "Improving interoperability of Virtual Commissioning toolchains by using OPC-UA-based technologies",
        "Virtual commissioning of a robotic cell: an educational case study",
        "Towards cloud-based virtual commissioning of distributed automation applications with IEC 61499 and containerization technology",
        "Real-time simulation and virtual commissioning of a modular robot system with OPC UA",
        "Standardized Framework for Evaluating Centralized and Decentralized Control Systems in Modular Assembly Systems"
    );

    let vpc_scopus = vec!(
        "A standardization approach to virtual commissioning strategies in complex production environments",
        "Novel approach to establish model-based development and virtual commissioning in practice",
        "Virtual commissioning of a robotized production cell with use of mechatronic features",
        "Mechatronic Swarm and its Virtual Commissioning",
        "Digital twin based virtual commissioning for computerized numerical control machine tools",
        "A digital twin-driven human-robot collaborative assembly-commissioning method for complex products",
        "A framework for an effective virtual commissioning of agent-based cyber-physical production systems integrated into manufacturing facilities",
        "A holistic approach for the development of a digital twin focused on commissioning and control of electromechanical feed axes",
        "Virtual commissioning as the main core of industry 4.0 - Case study in the automotive paint shop",
        "Towards cloud-based virtual commissioning of distributed automation applications with IEC 61499 and containerization technology",
        "Semi-automatic generation of a virtual representation of a production cell: Combining 3D CAD and VDI-2860 behavior models by means of AutomationML",
        "Virtual Commissioning Approach based on the Discrete Element Method",
        "Virtual comissioning of manufacturing system intelligent control",
        "Implementation of Digital Twin-based Virtual Commissioning in Machine Tool Manufacturing",
        "Hybrid Commissioning of Production Plants",
        "Engineering Method and Tool for the Complete Virtual Commissioning of Robotic Cells",
        "Digital twin-driven virtual commissioning of machine tool",
        "A Virtual Commissioning Selection Approach for Machine Automation",
        "Real-time simulation and virtual commissioning of a modular robot system with OPC UA",
        "Virtual Engineering and Commissioning to Support the Lifecycle of a Manufacturing Assembly System",
        "Validation of dynamic interoperability and virtual commissioning of production equipment in early development stages",
        "Simulation and virtual commissioning for intelligent buffer control system - Case study",
        "Interactive formal specification for efficient preparation of intelligent automation systems",
        "Modular Virtual Preparation method of production systems using a Digital Twin architecture",
        "Synchronizing digital process twins between virtual products and resources - A virtual design method",
        "Advanced tools for the control engineer in Industry 4.0",
        "Co-simulation-based virtual commissioning for modular process plants: Requirements, framework and support-toolchain for a virtual automation testing environment",
        "Automatic Generation of Charging Point's Digital Twin for Virtual Commissioning of Their Automation Systems",
        "Petri net controlled virtual commissioning - A virtual design-loop approach",
    );
    let mut vpc = vec!();
    let vpc_ieee_len = vpc_ieee.len();
    let vpc_scopus_len = vpc_scopus.len();
    vpc.extend(vpc_ieee);
    vpc.extend(vpc_scopus);
    let mut vpc_dedup: Vec<_> = vpc.into_iter().unique().collect();
    vpc_dedup.sort();
    // println!("Total: {}", vpc_ieee_len + vpc_scopus_len);
    // println!("Removed: {}", (vpc_ieee_len + vpc_scopus_len) - vpc_dedup.len());
    // println!("Dedup: {}", vpc_dedup.len());
    // for i in vpc_dedup {
    //     println!("{:?},", i)
    // }

    // VPC:
    // Total: 49
    // Removed: 10
    // Dedup: 39

    let vpc_final = vec!(
        "A Virtual Commissioning Selection Approach for Machine Automation",
        "A digital twin-driven human-robot collaborative assembly-commissioning method for complex products",
        "A framework for an effective virtual commissioning of agent-based cyber-physical production systems integrated into manufacturing facilities",
        "A holistic approach for the development of a digital twin focused on commissioning and control of electromechanical feed axes",
        "A standardization approach to virtual commissioning strategies in complex production environments",
        "Advanced tools for the control engineer in Industry 4.0",
        "Automatic Generation of Charging Point's Digital Twin for Virtual Commissioning of Their Automation Systems",
        "Co-simulation-based virtual commissioning for modular process plants: Requirements, framework and support-toolchain for a virtual automation testing environment",
        "Concept For An Interoperable Behavior Library For Virtual Commissioning Using PLCopenXML",
        "Digital twin based virtual commissioning for computerized numerical control machine tools",
        "Digital twin-driven virtual commissioning of machine tool",
        "Engineering Method and Tool for the Complete Virtual Commissioning of Robotic Cells",
        "Fault injection in Digital Twin as a means to test the response to process faults at virtual commissioning",
        "Hybrid Commissioning of Production Plants",
        "Implementation of Digital Twin-based Virtual Commissioning in Machine Tool Manufacturing",
        "Improving interoperability of Virtual Commissioning toolchains by using OPC-UA-based technologies",
        "Integrated virtual commissioning of a ROS2-based collaborative and intelligent automation system",
        "Interactive formal specification for efficient preparation of intelligent automation systems",
        "Manufacturing machine virtual commissioning: Automated validation of the control software",
        "Mechatronic Swarm and its Virtual Commissioning",
        "Modular Virtual Preparation method of production systems using a Digital Twin architecture",
        "Novel approach to establish model-based development and virtual commissioning in practice",
        "Petri net controlled virtual commissioning - A virtual design-loop approach",
        "Real-time simulation and virtual commissioning of a modular robot system with OPC UA",
        "Semi-automatic generation of a virtual representation of a production cell: Combining 3D CAD and VDI-2860 behavior models by means of AutomationML",
        "Simulation and virtual commissioning for intelligent buffer control system - Case study",
        "Standardized Framework for Evaluating Centralized and Decentralized Control Systems in Modular Assembly Systems",
        "Supporting the Design, Commissioning and Supervision of Smart Factory Components through their Digital Twin",
        "Synchronizing digital process twins between virtual products and resources - A virtual design method",
        "Towards cloud-based virtual commissioning of distributed automation applications with IEC 61499 and containerization technology",
        "Validation of dynamic interoperability and virtual commissioning of production equipment in early development stages",
        "Virtual Commissioning Approach based on the Discrete Element Method",
        "Virtual Commissioning Simulation as OpenAI Gym - A Reinforcement Learning Environment for Control Systems",
        "Virtual Engineering and Commissioning to Support the Lifecycle of a Manufacturing Assembly System",
        "Virtual Testing in Automated Driving Systems Certification. A Longitudinal Dynamics Validation Example",
        "Virtual comissioning of manufacturing system intelligent control",
        "Virtual commissioning as the main core of industry 4.0 - Case study in the automotive paint shop",
        "Virtual commissioning of a robotic cell: an educational case study",
        "Virtual commissioning of a robotized production cell with use of mechatronic features"
    );

    let idt_ieee = vec!(
        "A Framework for Using Data as an Engineering Tool for Sustainable Cyber-Physical Systems",
        "Let the Asset Decide: Digital Twins with Knowledge Graphs",
        "Architecture for Digital Twin implementation focusing on Industry 4.0",
        "A Digital Twin Based Industrial Automation and Control System Security Architecture",
        "Architecting Digital Twins",
        "A Novel Implementation Framework of Digital Twins for Intelligent Manufacturing Based on Container Technology and Cloud Manufacturing Services",
        "A Feature-Based Framework for Structuring Industrial Digital Twins",
        "Digital Twin and Industrial Internet of Things Architecture to Reduce Carbon Emissions",
        "A Digital Twin Framework for Industry 4.0 Enabling Next-Gen Manufacturing",
        "Industrial IoT and Digital Twins for a Smart Factory : An open source toolkit for application design and benchmarking",
    );

    let idt_scopus = vec!(
        "Flexible work cell simulator using digital twin methodology for highly complex systems in industry 4.0",
        "A hybrid simulation/optimization architecture for developing a digital twin",
        "The architecture development of Industry 4.0 compliant smart machine tool system (SMTS)",
        "Towards a digital twin platform for industrie 4.0",
        "Digital twin for smart manufacturing: a review of concepts towards a practical industrial implementation",
        "Empowering The Eclipse Arrowhead Framework with a Digital Twin as a Proxy Service",
        "Generic digital twin architecture for industrial energy systems",
        "A digital twin-based big data virtual and real fusion learning reference framework supported by industrial internet towards smart manufacturing",
        "Hybrid learning-based digital twin for manufacturing process: Modeling framework and implementation",
        "A Framework for Service-Oriented Digital Twin Systems for Discrete Workshops and Its Practical Case Study",
        "INDUSTRIAL COLLABORATIVE ROBOT DIGITAL TWIN INTEGRATION AND CONTROL USING ROBOT OPERATING SYSTEM",
        "Digital Twin: Toward the Integration Between System Design and RAMS Assessment Through the Model-Based Systems Engineering",
        "Digital twins in manufacturing: An assessment of key features",
        "Toward Intelligent Cyber-Physical Systems: Digital Twin Meets Artificial Intelligence",
        "Digital twin-based industrial cloud robotics: Framework, control approach and implementation",
        "Development of Digital twin for Plug-and-Produce of a Machine tending system through ISO 21919 interface",
        "A Hybrid Architecture of Digital Twin with Decision Support Layer for Industrial Maintenance",
        "A model-based Digital Twin to support responsive manufacturing systems",
        "A Service-based Architecture for the Interaction of Control and MES Systems in Industry 4.0 Environment",
        "A framework for development of digital twin industrial robot production lines based on a mechatronics approach",
        "Demonstration of an industrial framework for an implementation of a process digital twin",
        "Digital Twin and Industrial Internet of Things Architecture to Reduce Carbon Emissions",
        "Implementation of Digital Twin-based Virtual Commissioning in Machine Tool Manufacturing",
        "A Digital Twin Architecture Based on the Industrial Internet of Things Technologies",
        "Digital twin for adaptation of robots' behavior in flexible robotic assembly lines",
        "The Development of a Digital Twin Framework for an Industrial Robotic Drilling Process",
        "Towards a Novel Software Framework for the Intuitive Generation of Process Flows for Multiple Robotic Systems",
        "Digital Twins Approach for Sustainable Industry",
        "Digital Twin as Industrial Robots Manipulation Validation Tool",
        "Value Creation with Digital Twins: Application-Oriented Conceptual Framework and Case Study",
        "A digital twin framework of a material handling operator in industry 4.0 environments",
        "Virtual commissioning for advanced manufacturing using digital tools",
        "Digital Twin Enabled Smart Control Engineering as an Industrial AI: A New Framework and Case Study",
    );

    let mut idt= vec!();
    let idt_ieee_len = idt_ieee.len();
    let idt_scopus_len = idt_scopus.len();
    idt.extend(idt_ieee);
    idt.extend(idt_scopus);
    let mut idt_dedup: Vec<_> = idt.into_iter().unique().collect();
    idt_dedup.sort();
    // println!("Total: {}", idt_ieee_len + idt_scopus_len);
    // println!("Removed: {}", (idt_ieee_len + idt_scopus_len) - idt_dedup.len());
    // println!("Dedup: {}", idt_dedup.len());
    // for i in idt_dedup {
    //     println!("{:?},", i)
    // }


    // IDT:
    // Total: 43
    // Removed: 1
    // Dedup: 42


    let idt_final = vec!(
        "A Digital Twin Architecture Based on the Industrial Internet of Things Technologies",
        "A Digital Twin Based Industrial Automation and Control System Security Architecture",
        "A Digital Twin Framework for Industry 4.0 Enabling Next-Gen Manufacturing",
        "A Feature-Based Framework for Structuring Industrial Digital Twins",
        "A Framework for Service-Oriented Digital Twin Systems for Discrete Workshops and Its Practical Case Study",
        "A Framework for Using Data as an Engineering Tool for Sustainable Cyber-Physical Systems",
        "A Hybrid Architecture of Digital Twin with Decision Support Layer for Industrial Maintenance",
        "A Novel Implementation Framework of Digital Twins for Intelligent Manufacturing Based on Container Technology and Cloud Manufacturing Services",
        "A Service-based Architecture for the Interaction of Control and MES Systems in Industry 4.0 Environment",
        "A digital twin framework of a material handling operator in industry 4.0 environments",
        "A digital twin-based big data virtual and real fusion learning reference framework supported by industrial internet towards smart manufacturing",
        "A framework for development of digital twin industrial robot production lines based on a mechatronics approach",
        "A hybrid simulation/optimization architecture for developing a digital twin",
        "A model-based Digital Twin to support responsive manufacturing systems",
        "Architecting Digital Twins",
        "Architecture for Digital Twin implementation focusing on Industry 4.0",
        "Demonstration of an industrial framework for an implementation of a process digital twin",
        "Development of Digital twin for Plug-and-Produce of a Machine tending system through ISO 21919 interface",
        "Digital Twin Enabled Smart Control Engineering as an Industrial AI: A New Framework and Case Study",
        "Digital Twin and Industrial Internet of Things Architecture to Reduce Carbon Emissions",
        "Digital Twin as Industrial Robots Manipulation Validation Tool",
        "Digital Twin: Toward the Integration Between System Design and RAMS Assessment Through the Model-Based Systems Engineering",
        "Digital Twins Approach for Sustainable Industry",
        "Digital twin for adaptation of robots' behavior in flexible robotic assembly lines",
        "Digital twin for smart manufacturing: a review of concepts towards a practical industrial implementation",
        "Digital twin-based industrial cloud robotics: Framework, control approach and implementation",
        "Digital twins in manufacturing: An assessment of key features",
        "Empowering The Eclipse Arrowhead Framework with a Digital Twin as a Proxy Service",
        "Flexible work cell simulator using digital twin methodology for highly complex systems in industry 4.0",
        "Generic digital twin architecture for industrial energy systems",
        "Hybrid learning-based digital twin for manufacturing process: Modeling framework and implementation",
        "INDUSTRIAL COLLABORATIVE ROBOT DIGITAL TWIN INTEGRATION AND CONTROL USING ROBOT OPERATING SYSTEM",
        "Implementation of Digital Twin-based Virtual Commissioning in Machine Tool Manufacturing",
        "Industrial IoT and Digital Twins for a Smart Factory : An open source toolkit for application design and benchmarking",
        "Let the Asset Decide: Digital Twins with Knowledge Graphs",
        "The Development of a Digital Twin Framework for an Industrial Robotic Drilling Process",
        "The architecture development of Industry 4.0 compliant smart machine tool system (SMTS)",
        "Toward Intelligent Cyber-Physical Systems: Digital Twin Meets Artificial Intelligence",
        "Towards a Novel Software Framework for the Intuitive Generation of Process Flows for Multiple Robotic Systems",
        "Towards a digital twin platform for industrie 4.0",
        "Value Creation with Digital Twins: Application-Oriented Conceptual Framework and Case Study",
        "Virtual commissioning for advanced manufacturing using digital tools"
    );

    let imas_ieee = vec!(
        "Approaches to Creating a Multi-agent Architecture in the Industrial Internet of Things Systems",
        "Cooperative Control for Industrial Multi-agent Systems: Framework and Problems",
        "Multi-Agent Technology for Industrial Applications: Barriers and Trends",
        "Relaxing Platform Dependencies in Agent-Based Control Systems",
        "Cyber-physical production systems for SMEs-A generic multi agent based architecture and case study",
        "RoSA: A Framework for Modeling Self-Awareness in Cyber-Physical Systems"
    );

    let imas_scopus = vec!(
        "Integration challenges for the deployment of a multi-stage zero-defect manufacturing architecture",
        "Towards Integrating Multi-Agent Organizations in OPC UA for Developing Adaptive Cyber-Physical Systems",
        "Resilience enhancement through a multi-agent approach over cyber-physical systems",
        "Approaches to Creating a Multi-agent Architecture in the Industrial Internet of Things Systems",
        "Multi-agent Based IEC 61499 Function Block Modelling for Distributed Intelligent Automation",
        "An Switchable Multi-resolution Architecture of Cyber-Physical Manufacturing Systems (CPMS) for Industrial Robots Collaboration",
        "Cyber and Physical Systems Topology for the Industry 4.0 Smart Factory",
        "Architecting an agent-based fault diagnosis engine for IEC 61499 industrial cyber-physical systems",
        "The component-based design method for agent-based multi-AGV system",
        "A Distributed Multi-Agent Framework for Resilience Enhancement in Cyber-Physical Systems",
        "Model-driven design and development of flexible automated production control configurations for industry 4.0",
        "A fractal-theory-based multi-agent model of the cyber physical production system for customized products",
        "A game-theoretic method for resilient control design in industrial multi-agent CPSs with Markovian and coupled dynamics",
        "Multi-agent modeling of cyber-physical systems for IEC 61499 based distributed automation",
        "RMAS architecture for industrial agents in IEC 61499"
    );

    let mut imas = vec!();
    let imas_ieee_len = imas_ieee.len();
    let imas_scopus_len = imas_scopus.len();
    imas.extend(imas_ieee);
    imas.extend(imas_scopus);
    let mut imas_dedup: Vec<_> = imas.into_iter().unique().collect();
    imas_dedup.sort();
    // println!("Total: {}", imas_ieee_len + imas_scopus_len);
    // println!("Removed: {}", (imas_ieee_len + imas_scopus_len) - imas_dedup.len());
    // println!("Dedup: {}", imas_dedup.len());
    // for i in imas_dedup {
    //     println!("{:?},", i)
    // }


    // IMAS
    // Total: 21
    // Removed: 1
    // Dedup: 20


    let imas_final = vec!(
        "A Distributed Multi-Agent Framework for Resilience Enhancement in Cyber-Physical Systems",
        "A fractal-theory-based multi-agent model of the cyber physical production system for customized products",
        "A game-theoretic method for resilient control design in industrial multi-agent CPSs with Markovian and coupled dynamics",
        "An Switchable Multi-resolution Architecture of Cyber-Physical Manufacturing Systems (CPMS) for Industrial Robots Collaboration",
        "Approaches to Creating a Multi-agent Architecture in the Industrial Internet of Things Systems",
        "Architecting an agent-based fault diagnosis engine for IEC 61499 industrial cyber-physical systems",
        "Cooperative Control for Industrial Multi-agent Systems: Framework and Problems",
        "Cyber and Physical Systems Topology for the Industry 4.0 Smart Factory",
        "Cyber-physical production systems for SMEs-A generic multi agent based architecture and case study",
        "Integration challenges for the deployment of a multi-stage zero-defect manufacturing architecture",
        "Model-driven design and development of flexible automated production control configurations for industry 4.0",
        "Multi-Agent Technology for Industrial Applications: Barriers and Trends",
        "Multi-agent Based IEC 61499 Function Block Modelling for Distributed Intelligent Automation",
        "Multi-agent modeling of cyber-physical systems for IEC 61499 based distributed automation",
        "RMAS architecture for industrial agents in IEC 61499",
        "Relaxing Platform Dependencies in Agent-Based Control Systems",
        "Resilience enhancement through a multi-agent approach over cyber-physical systems",
        "RoSA: A Framework for Modeling Self-Awareness in Cyber-Physical Systems",
        "The component-based design method for agent-based multi-AGV system",
        "Towards Integrating Multi-Agent Organizations in OPC UA for Developing Adaptive Cyber-Physical Systems"
    );


    let cpps_ieee = vec!(
        "Research on the Architecture of Cyber-Physical Machine Tool System",
        "Pre-Integrated Architectures for sustainable complex Cyber-Physical Systems",
        "Utilizing an Enterprise Architecture Framework for Model-Based Industrial Systems Engineering",
        "Cyber-physical system architecture for machining production line",
        "A Hierarchical Domain-Specific Language for Cyber-physical Production Systems Integrating Asset Administration Shells",
        "Automation Software Architecture in CPPS - Definition, Challenges and Research Potentials",
        "Design of a flexible robot cell demonstrator based on CPPS concepts and technologies",
        "Cyber-Physical Machine Tools towards Digitalisation and Servitisation of Manufacturing",
        "Concept Design of a System Architecture for a Manufacturing Cyber-physical Digital Twin System",
        "SWAP-IT: A Scalable and Lightweight Industry 4.0 Architecture for Cyber-Physical Production Systems",
        "Testing Cyber-Physical Production System: Test Methods Categorization and dataset",
        "Codesign of Architecture, Control, and Scheduling of Modular Cyber-Physical Production Systems for Design Space Exploration",
        "Applying model-based Co-Simulation on modular Production Units in Complex Automation Systems",
        "A Reconfigurable Industry 4.0 Middleware Software Architecture",
        "Dynamically Wiring CPPS Software Architectures",
        "Cyber-Physical Systems for Industrial Applications",
        "System Architectures for Cyber-Physical Production Systems enabling Self-X and Autonomy",
        "A Hierarchical Meta-Model for the Design of Cyber-Physical Production Systems",
        "Cyber-physical microservices: An IoT-based framework for manufacturing systems",
        "Generic Autonomic Management as a Service in a SOA-based Framework for Industry 4.0",
        "Metamodeling of Cyber-Physical Production Systems using AutomationML for Collaborative Innovation",
        "Using a model-based engineering approach for developing Industrial Internet of Things applications",
        "Decentralized Autonomous Architecture for Resilient Cyber-Physical Production Systems",
        "Archer: An Event-Driven Architecture for Cyber-Physical Systems",
        "An Approach of Cyber-Physical Production Systems Architecture for Robot Control",
        "Design patterns for the implementation of Industrial Agent-based AASs",
        "A distributed control architecture for a reconfigurable manufacturing plant",
        "simbIoTe: A Simulator for Building Cyber-Physical System and Internet of Things Environments",
        "TiLA: Twin-in-the-Loop Architecture for Cyber-Physical Production Systems",
        "Integration of a formal specification approach into CPPS engineering workflow for machinery validation",
        "Supporting Skill-based Flexible Manufacturing with Symbolic AI Methods",
        "Evaluation of Cognitive Architectures for Cyber-Physical Production Systems",
        "Cyber-Physical Production Systems supported by Intelligent Devices (SmartBoxes) for Industrial Processes Digitalization",
        "Design of Flexible Cyber-Physical Production Systems Architecture for Industrial Robot Control",
        "A software defined hierarchical communication and data management architecture for industry 4.0",
        "An Integrated Design Method For Cyber-Physical Production Systems",
        "Multi-Agent Systems to Implement Industry 4.0 Components",
    );

    let cpps_scopus = vec!(
        "A Novel Cyber-physical production systems (CPPS) Architecture for Rapid Reconfiguration",
        "A hybrid modeling methodology for cyber physical production systems: framework and key techniques",
        "A Review on Programming Approaches for Dynamic Industrial Cyber Physical Systems",
        "Multi-paradigm modelling and co-simulation in prototyping a cyber-physical production system",
        "Resilient architecture for cyber-physical production systems",
        "Production Process Modelling Architecture to Support Improved Cyber-Physical Production Systems",
        "A product-process-resource based formal modelling framework for customized manufacturing in cyber-physical production systems",
        "Integrated Cyber Physical Simulation Modelling Environment for Manufacturing 4.0",
        "Design and Implementation of Runtime Verification Framework for Cyber-Physical Production Systems",
        "Architectural framework of digital twin-based cyber-physical production system for resilient rechargeable battery production",
        "Cyber-physical production systems architecture based on multi-agent's design pattern—comparison of selected approaches mapping four agent patterns",
        "DINASORE: A dynamic intelligent reconfiguration tool for cyber-physical production systems",
        "An approach of cyber-physical production systems architecture for robot control",
        "Assessing adaptability of software architectures for cyber physical production systems",
        "SWAP-IT: A Scalable and Lightweight Industry 4.0 Architecture for Cyber-Physical Production Systems",
        "Streamline 3D simulation model development for virtual commissioning with IEC61499",
        "Cyber-Physical System Implementation for Manufacturing With Analytics in the Cloud Layer",
        "An Architecture for Data Management, Visualisation and Supervision of Cyber- Physical Production Systems",
        "Codesign of Architecture, Control, and Scheduling of Modular Cyber-Physical Production Systems for Design Space Exploration",
        "A blockchain enabled Cyber-Physical System architecture for Industry 4.0 manufacturing systems",
        "Modular System Design Approach for Cyber Physical Production Systems",
        "Internet of things based cyber-physical system framework for real-time operations",
        "Decentralized Autonomous Architecture for Resilient Cyber-Physical Production Systems",
        "A Behaviour-Driven Development Approach for Cyber-Physical Production Systems",
        "A Cyber-Physical Machine Tools Platform using OPC UA and MTConnect",
        "An architecture of an Intelligent Digital Twin in a Cyber-Physical Production System",
        "Dynamically Wiring CPPS Software Architectures",
        "Cyber-physical system architecture for machining production line",
        "Integrated production and maintenance planning in cyber-physical production systems",
        "A Novel Architecture for Cyber-Physical Production Systems in Industry 4.0",
        "A Digital Twin Generic Architecture for Data-Driven Cyber-Physical Production Systems",
        "Cyber-Physical Manufacturing Systems for Industry 4.0: Architectural Approach and Pilot Case",
        "Placing the operator at the centre of Industry 4.0 design: Modelling and assessing human activities within cyber-physical systems",
        "Intelligent manufacturing system with human-cyber-physical fusion and collaboration for process fine control",
        "A synergic framework for cyber-physical production systems in the context of industry 4.0 and beyond",
        "Architecture and knowledge modelling for self-organized reconfiguration management of cyber-physical production systems",
        "Context-aware scheduling and control architecture for cyber-physical production systems",
        "Developing an engineering tool for Cyber-Physical Production Systems",
        "Digital twin-based cyber physical production system architectural framework for personalized production",
        "System Architectures for Cyber-Physical Production Systems enabling Self-X and Autonomy",
        "The framework design of smart factory in discrete manufacturing industry based on cyber-physical system",
        "Safe and secure system architectures for cyber-physical systems",
        "A Framework for Multidisciplinary Simulation of Cyber-Physical Production Systems",
        "Design of cyber physical system architecture for industry 4.0 through lean six sigma: conceptual foundations and research issues"
    );

    let mut cpps = vec!();
    let cpps_ieee_len = cpps_ieee.len();
    let cpps_scopus_len = cpps_scopus.len();
    cpps.extend(cpps_ieee);
    cpps.extend(cpps_scopus);
    let mut cpps_dedup: Vec<_> = cpps.into_iter().unique().collect();
    cpps_dedup.sort();
    // println!("Total: {}", cpps_ieee_len + cpps_scopus_len);
    // println!("Removed: {}", (cpps_ieee_len + cpps_scopus_len) - cpps_dedup.len());
    // println!("Dedup: {}", cpps_dedup.len());
    // for i in cpps_dedup {
    //     println!("{:?},", i)
    // }

    // CPPS
    // Total: 81
    // Removed: 6
    // Dedup: 75


    let cpps_final = vec!(
        "A Behaviour-Driven Development Approach for Cyber-Physical Production Systems",
        "A Cyber-Physical Machine Tools Platform using OPC UA and MTConnect",
        "A Digital Twin Generic Architecture for Data-Driven Cyber-Physical Production Systems",
        "A Framework for Multidisciplinary Simulation of Cyber-Physical Production Systems",
        "A Hierarchical Domain-Specific Language for Cyber-physical Production Systems Integrating Asset Administration Shells",
        "A Hierarchical Meta-Model for the Design of Cyber-Physical Production Systems",
        "A Novel Architecture for Cyber-Physical Production Systems in Industry 4.0",
        "A Novel Cyber-physical production systems (CPPS) Architecture for Rapid Reconfiguration",
        "A Reconfigurable Industry 4.0 Middleware Software Architecture",
        "A Review on Programming Approaches for Dynamic Industrial Cyber Physical Systems",
        "A blockchain enabled Cyber-Physical System architecture for Industry 4.0 manufacturing systems",
        "A distributed control architecture for a reconfigurable manufacturing plant",
        "A hybrid modeling methodology for cyber physical production systems: framework and key techniques",
        "A product-process-resource based formal modelling framework for customized manufacturing in cyber-physical production systems",
        "A software defined hierarchical communication and data management architecture for industry 4.0",
        "A synergic framework for cyber-physical production systems in the context of industry 4.0 and beyond",
        "An Approach of Cyber-Physical Production Systems Architecture for Robot Control",
        "An Architecture for Data Management, Visualisation and Supervision of Cyber- Physical Production Systems",
        "An Integrated Design Method For Cyber-Physical Production Systems",
        "An approach of cyber-physical production systems architecture for robot control",
        "An architecture of an Intelligent Digital Twin in a Cyber-Physical Production System",
        "Applying model-based Co-Simulation on modular Production Units in Complex Automation Systems",
        "Archer: An Event-Driven Architecture for Cyber-Physical Systems",
        "Architectural framework of digital twin-based cyber-physical production system for resilient rechargeable battery production",
        "Architecture and knowledge modelling for self-organized reconfiguration management of cyber-physical production systems",
        "Assessing adaptability of software architectures for cyber physical production systems",
        "Automation Software Architecture in CPPS - Definition, Challenges and Research Potentials",
        "Codesign of Architecture, Control, and Scheduling of Modular Cyber-Physical Production Systems for Design Space Exploration",
        "Concept Design of a System Architecture for a Manufacturing Cyber-physical Digital Twin System",
        "Context-aware scheduling and control architecture for cyber-physical production systems",
        "Cyber-Physical Machine Tools towards Digitalisation and Servitisation of Manufacturing",
        "Cyber-Physical Manufacturing Systems for Industry 4.0: Architectural Approach and Pilot Case",
        "Cyber-Physical Production Systems supported by Intelligent Devices (SmartBoxes) for Industrial Processes Digitalization",
        "Cyber-Physical System Implementation for Manufacturing With Analytics in the Cloud Layer",
        "Cyber-Physical Systems for Industrial Applications",
        "Cyber-physical microservices: An IoT-based framework for manufacturing systems",
        "Cyber-physical production systems architecture based on multi-agent's design pattern—comparison of selected approaches mapping four agent patterns",
        "Cyber-physical system architecture for machining production line",
        "DINASORE: A dynamic intelligent reconfiguration tool for cyber-physical production systems",
        "Decentralized Autonomous Architecture for Resilient Cyber-Physical Production Systems",
        "Design and Implementation of Runtime Verification Framework for Cyber-Physical Production Systems",
        "Design of Flexible Cyber-Physical Production Systems Architecture for Industrial Robot Control",
        "Design of a flexible robot cell demonstrator based on CPPS concepts and technologies",
        "Design of cyber physical system architecture for industry 4.0 through lean six sigma: conceptual foundations and research issues",
        "Design patterns for the implementation of Industrial Agent-based AASs",
        "Developing an engineering tool for Cyber-Physical Production Systems",
        "Digital twin-based cyber physical production system architectural framework for personalized production",
        "Dynamically Wiring CPPS Software Architectures",
        "Evaluation of Cognitive Architectures for Cyber-Physical Production Systems",
        "Generic Autonomic Management as a Service in a SOA-based Framework for Industry 4.0",
        "Integrated Cyber Physical Simulation Modelling Environment for Manufacturing 4.0",
        "Integrated production and maintenance planning in cyber-physical production systems",
        "Integration of a formal specification approach into CPPS engineering workflow for machinery validation",
        "Intelligent manufacturing system with human-cyber-physical fusion and collaboration for process fine control",
        "Internet of things based cyber-physical system framework for real-time operations",
        "Metamodeling of Cyber-Physical Production Systems using AutomationML for Collaborative Innovation",
        "Modular System Design Approach for Cyber Physical Production Systems",
        "Multi-Agent Systems to Implement Industry 4.0 Components",
        "Multi-paradigm modelling and co-simulation in prototyping a cyber-physical production system",
        "Placing the operator at the centre of Industry 4.0 design: Modelling and assessing human activities within cyber-physical systems",
        "Pre-Integrated Architectures for sustainable complex Cyber-Physical Systems",
        "Production Process Modelling Architecture to Support Improved Cyber-Physical Production Systems",
        "Research on the Architecture of Cyber-Physical Machine Tool System",
        "Resilient architecture for cyber-physical production systems",
        "SWAP-IT: A Scalable and Lightweight Industry 4.0 Architecture for Cyber-Physical Production Systems",
        "Safe and secure system architectures for cyber-physical systems",
        "Streamline 3D simulation model development for virtual commissioning with IEC61499",
        "Supporting Skill-based Flexible Manufacturing with Symbolic AI Methods",
        "System Architectures for Cyber-Physical Production Systems enabling Self-X and Autonomy",
        "Testing Cyber-Physical Production System: Test Methods Categorization and dataset",
        "The framework design of smart factory in discrete manufacturing industry based on cyber-physical system",
        "TiLA: Twin-in-the-Loop Architecture for Cyber-Physical Production Systems",
        "Using a model-based engineering approach for developing Industrial Internet of Things applications",
        "Utilizing an Enterprise Architecture Framework for Model-Based Industrial Systems Engineering",
        "simbIoTe: A Simulator for Building Cyber-Physical System and Internet of Things Environments"
    );

    let ifsr_ma_ieee = vec!(
        "Digital twin modeling method for CNC machine tool",
        "A Review on Cyber-Physical Systems: Models and Architectures",
        "Intelligent Manufacturing with Digital Twin",
        "Towards a Cloud-Based Controller for Data-Driven Service Orchestration in Smart Manufacturing",
        "A Reconfigurable Method for Intelligent Manufacturing Based on Industrial Cloud and Edge Intelligence",
        "Enhancing a Biological inspired Self-organized Architecture towards Smart Manufacturing",
        "A Novel Implementation Framework of Digital Twins for Intelligent Manufacturing Based on Container Technology and Cloud Manufacturing Services",
        "Application of Industrial Robot and Internet of Things in Intelligent Manufacturing System Supported by Software and Hardware",
        "A Reference Architecture Based on Edge and Cloud Computing for Smart Manufacturing",
        "Supervisory Control of Reconfigurable Manufacturing Systems based on NCESs",
        "Flexible Production Systems: Automated Generation of Operations Plans Based on ISA-95 and PDDL",
        "A Unified Digital Twin Framework for Real-time Monitoring and Evaluation of Smart Manufacturing Systems",
        "Supporting Skill-based Flexible Manufacturing with Symbolic AI Methods",
        "A Secure Fog Computing Architecture for IoT Based Smart Manufacturing System",
        "Towards IEC 61499 Based Distributed Intelligent Automation: Design and Computing Perspectives",
        "Toward Smart Manufacturing Using Spiral Digital Twin Framework and Twinchain",
        "Integrated virtual commissioning of a ROS2-based collaborative and intelligent automation system",
        "An Automatic Software Behavior Model Generation Method for Industrial Cyber-Physical System",
        "A Survey: Microservices Architecture in Advanced Manufacturing Systems",
        "Cooperative Robotics and Machine Learning for Smart Manufacturing: Platform Design and Trends Within the Context of Industrial Internet of Things",
        "Towards an OPC UA Compliant Programming Approach with Formal Model of Computation for Dynamic Reconfigurable Automation Systems",
        "A Cloud-Based Platform for Big Data-Driven CPS Modeling of Robots",
        "Knowledge-based formal modeling for CPPS in personalized intelligent manufacturing",
        "New IT Driven Service-Oriented Smart Manufacturing: Framework and Characteristics",
        "A Conceptual Architecture and Model for Smart Manufacturing Relying on Service-Based Digital Twins",
        "SDCWorks: A Formal Framework for Software Defined Control of Smart Manufacturing Systems",
        "Methodological Approach for Developing Reconfigurable Automation Systems",
        "Towards IEC 61499-Based Distributed Intelligent Automation: A Literature Review",
        "A Requirements Driven Digital Twin Framework: Specification and Opportunities",
        "A New Architecture for Controlling Smart Manufacturing Systems",
        "A distributed control architecture for a reconfigurable manufacturing plant",
        "Evaluating Skill-Based Control Architecture for Flexible Automation Systems",
        "System Architectures for Cyber-Physical Production Systems enabling Self-X and Autonomy",
        "Programming Abstractions for Simulation and Testing on Smart Manufacturing Systems",
        "A Digital Twin-based Approach Performing Integrated Process Planning and Scheduling for Service-based Production",
        "A Generic Plug & Produce System Composed of Semantic OPC UA Skills",
        "Implementation of Industrial Cyber Physical System: Challenges and Solutions",
        "A Feature Reserved Teaching Method for Pick-Place System under Robot Operating System",
        "Practical Guide to Smart Factory Transition Using IoT, Big Data and Edge Analytics",
        "A Review of the Literature on Smart Factory Implementation"
    );

    let ifsr_ma_scopus = vec!(
        "Digital twin framework for reconfigurable manufacturing systems (RMSs): design and simulation",
        "A New Architecture for Controlling Smart Manufacturing Systems",
        "Dynamic reconfiguration optimization of intelligent manufacturing system with human-robot collaboration based on digital twin",
        "A Hybrid Architecture for a Reconfigurable Cellular Remanufacturing System",
        "Developing an Improved Software Architecture Framework for Smart Manufacturing",
        "Development of the Architecture and Reconfiguration Methods for the Smart, Self-Reconfigurable Manufacturing System",
        "Practical use of robot manipulators as intelligent manufacturing systems",
        "A cyber-physical robotic mobile fulfillment system in smart manufacturing: The simulation aspect",
        "Supporting Skill-based Flexible Manufacturing with Symbolic AI Methods",
        "A digital-twin visualized architecture for Flexible Manufacturing System",
        "Sequence Planner: A Framework for Control of Intelligent Automation Systems",
        "A new approach to develop an intelligent manufacturing system using virtual tools",
        "A novel framework for intelligent automation",
        "Research on Intelligent Manufacturing System Model Based on Programmable Logic Controller",
        "A Cyber-Physical System Architecture for Smart Manufacturing",
        "Simulation framework for cyber-physical production system: Applying concept of LVC interoperation",
        "An Automatic Software Behavior Model Generation Method for Industrial Cyber-Physical System",
        "Research on real-time status visualization of intelligent manufacturing production line based on digital twin",
        "A Middleware Platform for Intelligent Automation: An Industrial Prototype Implementation",
        "Developing sensor signal-based digital twins for intelligent machine tools",
        "A generic tri-model-based approach for product-level digital twin development in a smart manufacturing environment",
        "Application of the sequence planner control framework to an intelligent automation system with a focus on error handling",
        "Discrete Event Simulation as a Robust Supporting Tool for Smart Manufacturing",
        "Data-driven framework to improve collaborative human-robot flexible manufacturing applications",
        "Intelligent Manufacturing with Digital Twin",
        "Development of an Event-Driven System Architecture for Smart Manufacturing",
        "Evaluating Skill-Based Control Architecture for Flexible Automation Systems",
        "A Development Methodology Framework of Smart Manufacturing Systems (Industry 4.0)",
        "A Cognitive Digital Twins Framework for Human-Robot Collaboration",
        "A Digital Twin Modular Framework for Reconfigurable Manufacturing Systems",
        "Programming Abstractions for Simulation and Testing on Smart Manufacturing Systems",
        "Application framework of digital twin-driven product smart manufacturing system: A case study of aeroengine blade manufacturing",
        "Toward Smart Manufacturing Using Spiral Digital Twin Framework and Twinchain",
        "Modular Design Method for Reconfigurable Manufacturing Systems",
        "A distributed control architecture for a reconfigurable manufacturing plant",
        "Development of a structural framework to improve reconfigurable manufacturing system adoption in the manufacturing industry",
        "A Novel Implementation Framework of Digital Twins for Intelligent Manufacturing Based on Container Technology and Cloud Manufacturing Services",
        "SDCWorks: A Formal Framework for Software Defined Control of Smart Manufacturing Systems",
        "Modeling and Optimization of Flexible Manufacturing Systems: A Stochastic Approach",
        "A hybrid simulation-based assessment framework of smart manufacturing systems",
        "Intelligent Automation Framework Using AI and RPA: An Introduction",
        "Towards the adoption of smart manufacturing systems: A development framework",
        "Modeling and object recognition skill transfer in industrial intelligent robots",
        "Digital Twin Framework for Reconfigurable Manufacturing Systems: Challenges and Requirements",
        "A Construction Method of Intelligent Manufacturing System under Industry 4.0 Model",
        "Knowledge-based formal modeling for CPPS in personalized intelligent manufacturing"
    );

    let mut ifsr_ma = vec!();
    let ifsr_ma_ieee_len = ifsr_ma_ieee.len();
    let ifsr_ma_scopus_len = ifsr_ma_scopus.len();
    ifsr_ma.extend(ifsr_ma_ieee);
    ifsr_ma.extend(ifsr_ma_scopus);
    let mut ifsr_ma_dedup: Vec<_> = ifsr_ma.into_iter().unique().collect();
    ifsr_ma_dedup.sort();
    // println!("Total: {}", ifsr_ma_ieee_len + ifsr_ma_scopus_len);
    // println!("Removed: {}", (ifsr_ma_ieee_len + ifsr_ma_scopus_len) - ifsr_ma_dedup.len());
    // println!("Dedup: {}", ifsr_ma_dedup.len());
    // for i in ifsr_ma_dedup {
    //     println!("{:?},", i)
    // }

    // IFSR_MA
    // Total: 87
    // Removed: 11
    // Dedup: 76


    let ifsr_ma_final = vec!(
        "A Cloud-Based Platform for Big Data-Driven CPS Modeling of Robots",
        "A Cognitive Digital Twins Framework for Human-Robot Collaboration",
        "A Conceptual Architecture and Model for Smart Manufacturing Relying on Service-Based Digital Twins",
        "A Construction Method of Intelligent Manufacturing System under Industry 4.0 Model",
        "A Cyber-Physical System Architecture for Smart Manufacturing",
        "A Development Methodology Framework of Smart Manufacturing Systems (Industry 4.0)",
        "A Digital Twin Modular Framework for Reconfigurable Manufacturing Systems",
        "A Digital Twin-based Approach Performing Integrated Process Planning and Scheduling for Service-based Production",
        "A Feature Reserved Teaching Method for Pick-Place System under Robot Operating System",
        "A Generic Plug & Produce System Composed of Semantic OPC UA Skills",
        "A Hybrid Architecture for a Reconfigurable Cellular Remanufacturing System",
        "A Middleware Platform for Intelligent Automation: An Industrial Prototype Implementation",
        "A New Architecture for Controlling Smart Manufacturing Systems",
        "A Novel Implementation Framework of Digital Twins for Intelligent Manufacturing Based on Container Technology and Cloud Manufacturing Services",
        "A Reconfigurable Method for Intelligent Manufacturing Based on Industrial Cloud and Edge Intelligence",
        "A Reference Architecture Based on Edge and Cloud Computing for Smart Manufacturing",
        "A Requirements Driven Digital Twin Framework: Specification and Opportunities",
        "A Review of the Literature on Smart Factory Implementation",
        "A Review on Cyber-Physical Systems: Models and Architectures",
        "A Secure Fog Computing Architecture for IoT Based Smart Manufacturing System",
        "A Survey: Microservices Architecture in Advanced Manufacturing Systems",
        "A Unified Digital Twin Framework for Real-time Monitoring and Evaluation of Smart Manufacturing Systems",
        "A cyber-physical robotic mobile fulfillment system in smart manufacturing: The simulation aspect",
        "A digital-twin visualized architecture for Flexible Manufacturing System",
        "A distributed control architecture for a reconfigurable manufacturing plant",
        "A generic tri-model-based approach for product-level digital twin development in a smart manufacturing environment",
        "A hybrid simulation-based assessment framework of smart manufacturing systems",
        "A new approach to develop an intelligent manufacturing system using virtual tools",
        "A novel framework for intelligent automation",
        "An Automatic Software Behavior Model Generation Method for Industrial Cyber-Physical System",
        "Application framework of digital twin-driven product smart manufacturing system: A case study of aeroengine blade manufacturing",
        "Application of Industrial Robot and Internet of Things in Intelligent Manufacturing System Supported by Software and Hardware",
        "Application of the sequence planner control framework to an intelligent automation system with a focus on error handling",
        "Cooperative Robotics and Machine Learning for Smart Manufacturing: Platform Design and Trends Within the Context of Industrial Internet of Things",
        "Data-driven framework to improve collaborative human-robot flexible manufacturing applications",
        "Developing an Improved Software Architecture Framework for Smart Manufacturing",
        "Developing sensor signal-based digital twins for intelligent machine tools",
        "Development of a structural framework to improve reconfigurable manufacturing system adoption in the manufacturing industry",
        "Development of an Event-Driven System Architecture for Smart Manufacturing",
        "Development of the Architecture and Reconfiguration Methods for the Smart, Self-Reconfigurable Manufacturing System",
        "Digital Twin Framework for Reconfigurable Manufacturing Systems: Challenges and Requirements",
        "Digital twin framework for reconfigurable manufacturing systems (RMSs): design and simulation",
        "Digital twin modeling method for CNC machine tool",
        "Discrete Event Simulation as a Robust Supporting Tool for Smart Manufacturing",
        "Dynamic reconfiguration optimization of intelligent manufacturing system with human-robot collaboration based on digital twin",
        "Enhancing a Biological inspired Self-organized Architecture towards Smart Manufacturing",
        "Evaluating Skill-Based Control Architecture for Flexible Automation Systems",
        "Flexible Production Systems: Automated Generation of Operations Plans Based on ISA-95 and PDDL",
        "Implementation of Industrial Cyber Physical System: Challenges and Solutions",
        "Implementation of industrial cyber physical system: Challenges and solutions",
        "Integrated virtual commissioning of a ROS2-based collaborative and intelligent automation system",
        "Intelligent Automation Framework Using AI and RPA: An Introduction",
        "Intelligent Manufacturing with Digital Twin",
        "Knowledge-based formal modeling for CPPS in personalized intelligent manufacturing",
        "Methodological Approach for Developing Reconfigurable Automation Systems",
        "Modeling and Optimization of Flexible Manufacturing Systems: A Stochastic Approach",
        "Modeling and object recognition skill transfer in industrial intelligent robots",
        "Modular Design Method for Reconfigurable Manufacturing Systems",
        "New IT Driven Service-Oriented Smart Manufacturing: Framework and Characteristics",
        "Practical Guide to Smart Factory Transition Using IoT, Big Data and Edge Analytics",
        "Practical use of robot manipulators as intelligent manufacturing systems",
        "Programming Abstractions for Simulation and Testing on Smart Manufacturing Systems",
        "Research on Intelligent Manufacturing System Model Based on Programmable Logic Controller",
        "Research on real-time status visualization of intelligent manufacturing production line based on digital twin",
        "SDCWorks: A Formal Framework for Software Defined Control of Smart Manufacturing Systems",
        "Sequence Planner: A Framework for Control of Intelligent Automation Systems",
        "Simulation framework for cyber-physical production system: Applying concept of LVC interoperation",
        "Supervisory Control of Reconfigurable Manufacturing Systems based on NCESs",
        "Supporting Skill-based Flexible Manufacturing with Symbolic AI Methods",
        "System Architectures for Cyber-Physical Production Systems enabling Self-X and Autonomy",
        "Toward Smart Manufacturing Using Spiral Digital Twin Framework and Twinchain",
        "Towards IEC 61499 Based Distributed Intelligent Automation: Design and Computing Perspectives",
        "Towards IEC 61499-Based Distributed Intelligent Automation: A Literature Review",
        "Towards a Cloud-Based Controller for Data-Driven Service Orchestration in Smart Manufacturing",
        "Towards an OPC UA Compliant Programming Approach with Formal Model of Computation for Dynamic Reconfigurable Automation Systems",
        "Towards the adoption of smart manufacturing systems: A development framework"
    );

    let ros_ieee = vec!(
        "A Modeling Tool for Reconfigurable Skills in ROS",
        "Design Patterns in Building Software for a Middle Size League Robot Using Robot Operating System",
        "Behavioral Analysis of ROS motion planners integrated with Robotics Middleware Framework (RMF)",
        "A Self-Driving Car Architecture in ROS2",
        "CompROS: A composable ROS2 based architecture for real-time embedded robotic development",
        "The High-Assurance ROS Framework",
        "ROS-MAGNA, a ROS-based framework for the definition and management of multi-UAS cooperative missions",
        "Documentation and Modeling of ROS Systems",
        "Interoperability Between ROS and OPC UA: A Local Cloud-Based Approach",
        "Generic ROS-based Architecture for Heterogeneous Multi-Autonomous Systems Development",
        "A Container-Based Framework for Developing ROS Applications",
        "Pushing ROS towards the Dark Side: A ROS-based Co-Simulation Architecture for Mixed-Reality Test Systems for Autonomous Vehicles",
        "A Software Architecture for Service Robots Manipulating Objects in Human Environments",
        "Modular ROS-based software architecture for reconfigurable, Industry 4.0 compatible robotic workcells",
        "A Robotic Control System Using Robot Operating System and MATLAB for Sensor Fusion and Human-Robot Interaction",
        "Design of a Framework for Implementation of Industrial Robot Manipulation Using PLC and ROS 2",
        "A ROS2-Based Framework for Industrial Automation Systems",
        "A Recommendation for a Systems Engineering Process and System Architecture for UAS",
        "Implementation of PLC controller connected Gazebo-ROS to support IEC 61131-3",
        "Integrated virtual commissioning of a ROS2-based collaborative and intelligent automation system",
        "Development and Deployment of Complex Robotic Applications using Containerized Infrastructures",
        "Behavior Trees based Flexible Task Planner Built on ROS2 Framework",
        "Generating ROS-based Software for Industrial Cyber-Physical Systems from UML/MARTE",
        "RobotAtFactory 4.0: a ROS framework for the SimTwo simulator",
        "Verification of system-wide safety properties of ROS applications",
        "A Formal Model-Based Design Method for Robotic Systems",
        "Modular ROS Based Autonomous Mobile Industrial Robot System for Automated Intelligent Manufacturing Applications",
        "PlanSys2: A Planning System Framework for ROS2",
        "P&P - Standard architecture to enable fast software prototyping for robot arms",
        "Distributed and Synchronized Setup towards Real-Time Robotic Control using ROS2 on Linux",
        "Control components for Collaborative and Intelligent Automation Systems",
        "MAFOSS: Multi-Agent Framework using Open-Source Software",
        "ROSIE: A ROS Adapter for a Modular Digital Twinning Framework",
        "KubeROS: A Unified Platform for Automated and Scalable Deployment of ROS2-based Multi-Robot Applications",
        "Efficient Industrial Solution for Robotic Task Sequencing Problem With Mutual Collision Avoidance & Cycle Time Optimization"
    );

    let ros_scopus = vec!(
        "PRobPlan: A Framework of Integrating Probabilistic Planning into ROS",
        "A Modeling Tool for Reconfigurable Skills in ROS",
        "Developing Production-Grade Applications with ROS 2",
        "Developing a Testing Framework for Cyber-Physical Systems using Gazebo",
        "ROS-based architecture for fast digital twin development of smart manufacturing robotized systems",
        "A ROS-Based Framework for Simulation and Benchmarking of Multi-robot Patrolling Algorithms",
        "Formal Verification of ROS Based Systems Using a Linear Logic Theorem Prover",
        "A multi-agent simulator environment based on the robot operating system for human-robot interaction applications",
        "Optimized execution of PDDL plans using behavior trees",
        "Probabilistic Planning for Robotics with ROSPlan",
        "A formal model-based design method for robotic systems",
        "A planning system for robot sampling task-based on ROS framework",
        "Planning and acting framework under robot operating system",
        "Lab-Scale Smart Factory Implementation Using ROS",
        "ROS-Based Multi-Robot System Simulator",
        "A Feature Reserved Teaching Method for Pick-Place System under Robot Operating System",
        "Event-Driven Programming of FPGA-accelerated ROS 2 Robotics Applications",
        "Applying MDE to ROS systems: A comparative analysis",
        "ROS-MAGNA, a ROS-based framework for the definition and management of multi-UAS cooperative missions",
        "An Advanced Human-Robot Interaction Interface for Collaborative Robotic Assembly Tasks",
        "The PoundCloud framework for ROS-based cloud robotics: Case studies on autonomous navigation and human-robot interaction",
        "A Robotic Control System Using Robot Operating System and MATLAB for Sensor Fusion and Human-Robot Interaction",
        "MAFOSS: Multi-Agent Framework using Open-Source Software",
        "The High-Assurance ROS Framework",
        "Digital twin for adaptation of robots behavior in flexible robotic assembly lines",
        "Let there be intelligence!- A novel cognitive architecture for teaching assistant social robots",
        "A ROS2 based communication architecture for control in collaborative and intelligent automation systems",
        "A ROS2-Based Framework for Industrial Automation Systems",
        "ROS-based robot simulation in human-robot collaboration",
        "Towards a real-time BDI model for ROS 2",
        "Interoperability Between ROS and OPC UA: A Local Cloud-Based Approach",
        "Virtual experimental investigation for industrial robotics in gazebo environment",
        "Generic ROS-based Architecture for Heterogeneous Multi-Autonomous Systems Development",
        "A robot augmented environment based on ROS multi-agent structure",
        "Software-in-the-Loop Modeling and Simulation Framework for Autonomous Vehicles",
        "Simulation Environment for Modeling and Testing of Autonomous Assembly in Space for Multiple Robotic Arms",
        "ROS and cFS System (RACS): Easing Space Robotic Development",
        "Design and Development of a Research Oriented Low Cost Robotics Platform with a Novel Dynamic Global Path Planning Approach",
        "ROS-TiPlEx: How to make experts in A.I. Planning and Robotics talk together and be happy",
        "Distributed and Synchronized Setup towards Real-Time Robotic Control using ROS2 on Linux",
        "Generating Safe Autonomous Decision-Making in ROS",
        "Teaching robotics with robot operating system (ROS): A behavior model perspective",
        "Development of Intelligent Behaviors for Social Robots via User-Friendly and Modular Programming Tools",
        "Decentralized Task and Path Planning for Multi-Robot Systems",
        "RobotAtFactory 4.0: a ROS framework for the SimTwo simulator",
        "A Container-Based Framework for Developing ROS Applications",
        "Modular ROS-based software architecture for reconfigurable, Industry 4.0 compatible robotic workcells",
        "Leveraging domain specific modeling to increase accessibility of robot programming",
        "Requirements for designing and controlling autonomous collaborative robots system-an industrial case",
        "Flow-Based ROS2 Programming Environment for Control Drone",
        "A Demonstration of BDI-Based Robotic Systems with ROS2",
        "Skill-Based Easy Programming Interface for Industrial Applications",
        "How to build and validate a safe and reliable Autonomous Driving stack? A ROS based software modular architecture baseline",
        "ROS-Industrial based robotic cell for Industry 4.0: Eye-in-hand stereo camera and visual servoing for flexible, fast, and accurate picking and hooking in the production line",
        "Behavior Trees based Flexible Task Planner Built on ROS2 Framework",
        "Design and Implementation of Software Based on Visual Simulation for Cargo Picking and Placing of Space Manipulator",
        "CompROS: A composable ROS2 based architecture for real-time embedded robotic development",
        "RoBMEX: ROS-based modelling framework for end-users and experts",
        "Generating ROS-based Software for Industrial Cyber-Physical Systems from UML/MARTE",
        "A software framework to create behaviors for androids and its implementation on the mobile android ibuki",
        "Property-based testing for the robot operating system",
        "Implementation of PLC controller connected Gazebo-ROS to support IEC 61131-3",
        "Analyzing a ROS based architecture for its cross reuse in ISO26262 settings",
        "PlanSys2: A Planning System Framework for ROS2",
        "Repeatable Decentralized Simulations for Cyber-Physical Systems",
        "Combining self-organisation with decision-making and planning"
    );

    let mut ros = vec!();
    let ros_ieee_len = ros_ieee.len();
    let ros_scopus_len = ros_scopus.len();
    ros.extend(ros_ieee);
    ros.extend(ros_scopus);
    let mut ros_dedup: Vec<_> = ros.into_iter().unique().collect();
    ros_dedup.sort();
    // println!("Total: {}", ros_ieee_len + ros_scopus_len);
    // println!("Removed: {}", (ros_ieee_len + ros_scopus_len) - ros_dedup.len());
    // println!("Dedup: {}", ros_dedup.len());
    // for i in ros_dedup {
    //     println!("{:?},", i)
    // }

    // ROS
    // Total: 101
    // Removed: 17
    // Dedup: 84


    let ros_final = vec!(
        "A Container-Based Framework for Developing ROS Applications",
        "A Demonstration of BDI-Based Robotic Systems with ROS2",
        "A Feature Reserved Teaching Method for Pick-Place System under Robot Operating System",
        "A Formal Model-Based Design Method for Robotic Systems",
        "A Modeling Tool for Reconfigurable Skills in ROS",
        "A ROS-Based Framework for Simulation and Benchmarking of Multi-robot Patrolling Algorithms",
        "A ROS2 based communication architecture for control in collaborative and intelligent automation systems",
        "A ROS2-Based Framework for Industrial Automation Systems",
        "A Recommendation for a Systems Engineering Process and System Architecture for UAS",
        "A Robotic Control System Using Robot Operating System and MATLAB for Sensor Fusion and Human-Robot Interaction",
        "A Self-Driving Car Architecture in ROS2",
        "A Software Architecture for Service Robots Manipulating Objects in Human Environments",
        "A formal model-based design method for robotic systems",
        "A multi-agent simulator environment based on the robot operating system for human-robot interaction applications",
        "A planning system for robot sampling task-based on ROS framework",
        "A robot augmented environment based on ROS multi-agent structure",
        "A software framework to create behaviors for androids and its implementation on the mobile android ibuki",
        "An Advanced Human-Robot Interaction Interface for Collaborative Robotic Assembly Tasks",
        "Analyzing a ROS based architecture for its cross reuse in ISO26262 settings",
        "Applying MDE to ROS systems: A comparative analysis",
        "Behavior Trees based Flexible Task Planner Built on ROS2 Framework",
        "Behavioral Analysis of ROS motion planners integrated with Robotics Middleware Framework (RMF)",
        "Combining self-organisation with decision-making and planning",
        "CompROS: A composable ROS2 based architecture for real-time embedded robotic development",
        "Control components for Collaborative and Intelligent Automation Systems",
        "Decentralized Task and Path Planning for Multi-Robot Systems",
        "Design Patterns in Building Software for a Middle Size League Robot Using Robot Operating System",
        "Design and Development of a Research Oriented Low Cost Robotics Platform with a Novel Dynamic Global Path Planning Approach",
        "Design and Implementation of Software Based on Visual Simulation for Cargo Picking and Placing of Space Manipulator",
        "Design of a Framework for Implementation of Industrial Robot Manipulation Using PLC and ROS 2",
        "Developing Production-Grade Applications with ROS 2",
        "Developing a Testing Framework for Cyber-Physical Systems using Gazebo",
        "Development and Deployment of Complex Robotic Applications using Containerized Infrastructures",
        "Development of Intelligent Behaviors for Social Robots via User-Friendly and Modular Programming Tools",
        "Digital twin for adaptation of robots behavior in flexible robotic assembly lines",
        "Distributed and Synchronized Setup towards Real-Time Robotic Control using ROS2 on Linux",
        "Documentation and Modeling of ROS Systems",
        "Efficient Industrial Solution for Robotic Task Sequencing Problem With Mutual Collision Avoidance & Cycle Time Optimization",
        "Event-Driven Programming of FPGA-accelerated ROS 2 Robotics Applications",
        "Flow-Based ROS2 Programming Environment for Control Drone",
        "Formal Verification of ROS Based Systems Using a Linear Logic Theorem Prover",
        "Generating ROS-based Software for Industrial Cyber-Physical Systems from UML/MARTE",
        "Generating Safe Autonomous Decision-Making in ROS",
        "Generic ROS-based Architecture for Heterogeneous Multi-Autonomous Systems Development",
        "How to build and validate a safe and reliable Autonomous Driving stack? A ROS based software modular architecture baseline",
        "Implementation of PLC controller connected Gazebo-ROS to support IEC 61131-3",
        "Integrated virtual commissioning of a ROS2-based collaborative and intelligent automation system",
        "Interoperability Between ROS and OPC UA: A Local Cloud-Based Approach",
        "KubeROS: A Unified Platform for Automated and Scalable Deployment of ROS2-based Multi-Robot Applications",
        "Lab-Scale Smart Factory Implementation Using ROS",
        "Let there be intelligence!- A novel cognitive architecture for teaching assistant social robots",
        "Leveraging domain specific modeling to increase accessibility of robot programming",
        "MAFOSS: Multi-Agent Framework using Open-Source Software",
        "Modular ROS Based Autonomous Mobile Industrial Robot System for Automated Intelligent Manufacturing Applications",
        "Modular ROS-based software architecture for reconfigurable, Industry 4.0 compatible robotic workcells",
        "Optimized execution of PDDL plans using behavior trees",
        "P&P - Standard architecture to enable fast software prototyping for robot arms",
        "PRobPlan: A Framework of Integrating Probabilistic Planning into ROS",
        "PlanSys2: A Planning System Framework for ROS2",
        "Planning and acting framework under robot operating system",
        "Probabilistic Planning for Robotics with ROSPlan",
        "Property-based testing for the robot operating system",
        "Pushing ROS towards the Dark Side: A ROS-based Co-Simulation Architecture for Mixed-Reality Test Systems for Autonomous Vehicles",
        "ROS and cFS System (RACS): Easing Space Robotic Development",
        "ROS-Based Multi-Robot System Simulator",
        "ROS-Industrial based robotic cell for Industry 4.0: Eye-in-hand stereo camera and visual servoing for flexible, fast, and accurate picking and hooking in the production line",
        "ROS-MAGNA, a ROS-based framework for the definition and management of multi-UAS cooperative missions",
        "ROS-TiPlEx: How to make experts in A.I. Planning and Robotics talk together and be happy",
        "ROS-based architecture for fast digital twin development of smart manufacturing robotized systems",
        "ROS-based robot simulation in human-robot collaboration",
        "ROSIE: A ROS Adapter for a Modular Digital Twinning Framework",
        "Repeatable Decentralized Simulations for Cyber-Physical Systems",
        "Requirements for designing and controlling autonomous collaborative robots system-an industrial case",
        "RoBMEX: ROS-based modelling framework for end-users and experts",
        "RobotAtFactory 4.0: a ROS framework for the SimTwo simulator",
        "Simulation Environment for Modeling and Testing of Autonomous Assembly in Space for Multiple Robotic Arms",
        "Skill-Based Easy Programming Interface for Industrial Applications",
        "Software-in-the-Loop Modeling and Simulation Framework for Autonomous Vehicles",
        "Teaching robotics with robot operating system (ROS): A behavior model perspective",
        "The High-Assurance ROS Framework",
        "The PoundCloud framework for ROS-based cloud robotics: Case studies on autonomous navigation and human-robot interaction",
        "Towards a real-time BDI model for ROS 2",
        "Verification of system-wide safety properties of ROS applications",
        "Virtual experimental investigation for industrial robotics in gazebo environment"
    );

    let mut total = vec!();
    let total_len =  vpc_final.len() + idt_final.len() + imas_final.len() + cpps_final.len() + ifsr_ma_final.len() + ros_final.len();
    total.extend(vpc_final);
    total.extend(idt_final);
    total.extend(imas_final);
    total.extend(cpps_final);
    total.extend(ifsr_ma_final);
    total.extend(ros_final);
    let mut total_dedup: Vec<_> = total.into_iter().unique().collect();
    total_dedup.sort();
    println!("Total: {}", total_len);
    println!("Removed: {}", total_len - total_dedup.len());
    println!("Dedup: {}", total_dedup.len());
    for i in total_dedup {
        println!("{:?},", i)
    }

    let stage_2_final = vec!(
        // "A Behaviour-Driven Development Approach for Cyber-Physical Production Systems", // ok
        // "A Cloud-Based Platform for Big Data-Driven CPS Modeling of Robots", // no
        // "A Cognitive Digital Twins Framework for Human-Robot Collaboration", // no
        // "A Conceptual Architecture and Model for Smart Manufacturing Relying on Service-Based Digital Twins", // no
        // "A Construction Method of Intelligent Manufacturing System under Industry 4.0 Model", // no
        // "A Container-Based Framework for Developing ROS Applications", // ok
        // "A Cyber-Physical Machine Tools Platform using OPC UA and MTConnect", // no
        // "A Cyber-Physical System Architecture for Smart Manufacturing", // no
        // "A Demonstration of BDI-Based Robotic Systems with ROS2", // ctrl
        // "A Development Methodology Framework of Smart Manufacturing Systems (Industry 4.0)", // no
        // "A Digital Twin Architecture Based on the Industrial Internet of Things Technologies", // no
        // "A Digital Twin Based Industrial Automation and Control System Security Architecture", // no
        // "A Digital Twin Framework for Industry 4.0 Enabling Next-Gen Manufacturing", // no
        // "A Digital Twin Generic Architecture for Data-Driven Cyber-Physical Production Systems", // no
        // "A Digital Twin Modular Framework for Reconfigurable Manufacturing Systems", // no
        // "A Digital Twin-based Approach Performing Integrated Process Planning and Scheduling for Service-based Production", // no
        // "A Distributed Multi-Agent Framework for Resilience Enhancement in Cyber-Physical Systems", // no
        // "A Feature Reserved Teaching Method for Pick-Place System under Robot Operating System", // no
        // "A Feature-Based Framework for Structuring Industrial Digital Twins", // no
        // "A Formal Model-Based Design Method for Robotic Systems", // ok
        // "A Framework for Multidisciplinary Simulation of Cyber-Physical Production Systems", // close
        // "A Framework for Service-Oriented Digital Twin Systems for Discrete Workshops and Its Practical Case Study", // ok
        // "A Framework for Using Data as an Engineering Tool for Sustainable Cyber-Physical Systems", // no
        // "A Generic Plug & Produce System Composed of Semantic OPC UA Skills", // ctrl
        // "A Hierarchical Domain-Specific Language for Cyber-physical Production Systems Integrating Asset Administration Shells", // no
        // "A Hierarchical Meta-Model for the Design of Cyber-Physical Production Systems", // no
        // "A Hybrid Architecture for a Reconfigurable Cellular Remanufacturing System", // no
        // "A Hybrid Architecture of Digital Twin with Decision Support Layer for Industrial Maintenance", // no
        // "A Middleware Platform for Intelligent Automation: An Industrial Prototype Implementation", // ctrl
        // "A Modeling Tool for Reconfigurable Skills in ROS", // no
        // "A New Architecture for Controlling Smart Manufacturing Systems", // no
        // "A Novel Architecture for Cyber-Physical Production Systems in Industry 4.0", // no
        // "A Novel Cyber-physical production systems (CPPS) Architecture for Rapid Reconfiguration", // no
        // "A Novel Implementation Framework of Digital Twins for Intelligent Manufacturing Based on Container Technology and Cloud Manufacturing Services", // ok
        // "A ROS-Based Framework for Simulation and Benchmarking of Multi-robot Patrolling Algorithms", // no
        // "A ROS2 based communication architecture for control in collaborative and intelligent automation systems", // no
        // "A ROS2-Based Framework for Industrial Automation Systems", // no
        // "A Recommendation for a Systems Engineering Process and System Architecture for UAS", // no
        // "A Reconfigurable Industry 4.0 Middleware Software Architecture", // no
        // "A Reconfigurable Method for Intelligent Manufacturing Based on Industrial Cloud and Edge Intelligence", // no
        // "A Reference Architecture Based on Edge and Cloud Computing for Smart Manufacturing", // no
        // "A Requirements Driven Digital Twin Framework: Specification and Opportunities", // no
        // "A Review of the Literature on Smart Factory Implementation", // no
        // "A Review on Cyber-Physical Systems: Models and Architectures",  // no
        // "A Review on Programming Approaches for Dynamic Industrial Cyber Physical Systems", // no
        // "A Robotic Control System Using Robot Operating System and MATLAB for Sensor Fusion and Human-Robot Interaction", // no
        // "A Secure Fog Computing Architecture for IoT Based Smart Manufacturing System", // no
        // "A Self-Driving Car Architecture in ROS2", // no
        // "A Service-based Architecture for the Interaction of Control and MES Systems in Industry 4.0 Environment", // no
        // "A Software Architecture for Service Robots Manipulating Objects in Human Environments", // ctrl
        // "A Survey: Microservices Architecture in Advanced Manufacturing Systems", // no
        // "A Unified Digital Twin Framework for Real-time Monitoring and Evaluation of Smart Manufacturing Systems", // close
        // "A Virtual Commissioning Selection Approach for Machine Automation", // vc
        // "A blockchain enabled Cyber-Physical System architecture for Industry 4.0 manufacturing systems", // no
        // "A cyber-physical robotic mobile fulfillment system in smart manufacturing: The simulation aspect", // no
        // "A digital twin framework of a material handling operator in industry 4.0 environments", // no
        // "A digital twin-based big data virtual and real fusion learning reference framework supported by industrial internet towards smart manufacturing", // no
        // "A digital twin-driven human-robot collaborative assembly-commissioning method for complex products", // no
        // "A digital-twin visualized architecture for Flexible Manufacturing System", // no
        // "A distributed control architecture for a reconfigurable manufacturing plant", // no
        // "A fractal-theory-based multi-agent model of the cyber physical production system for customized products",  // no
        // "A framework for an effective virtual commissioning of agent-based cyber-physical production systems integrated into manufacturing facilities", // ok
        // "A framework for development of digital twin industrial robot production lines based on a mechatronics approach",  // no
        // "A game-theoretic method for resilient control design in industrial multi-agent CPSs with Markovian and coupled dynamics", // no
        // "A generic tri-model-based approach for product-level digital twin development in a smart manufacturing environment", // no
        // "A holistic approach for the development of a digital twin focused on commissioning and control of electromechanical feed axes", // no
        // "A hybrid modeling methodology for cyber physical production systems: framework and key techniques",  // ok
        // "A hybrid simulation-based assessment framework of smart manufacturing systems",  // no
        // "A hybrid simulation/optimization architecture for developing a digital twin", // no
        // "A model-based Digital Twin to support responsive manufacturing systems", // no
        // "A multi-agent simulator environment based on the robot operating system for human-robot interaction applications", // no
        // "A new approach to develop an intelligent manufacturing system using virtual tools", // no
        // "A novel framework for intelligent automation", // no
        // "A planning system for robot sampling task-based on ROS framework",  // no
        // "A product-process-resource based formal modelling framework for customized manufacturing in cyber-physical production systems", // ok
        // "A robot augmented environment based on ROS multi-agent structure", // no 
        // "A software defined hierarchical communication and data management architecture for industry 4.0", // no 
        // "A software framework to create behaviors for androids and its implementation on the mobile android ibuki", // no
        // "A standardization approach to virtual commissioning strategies in complex production environments", // no 
        // "A synergic framework for cyber-physical production systems in the context of industry 4.0 and beyond", // no
        // "Advanced tools for the control engineer in Industry 4.0", // ok
        // "An Advanced Human-Robot Interaction Interface for Collaborative Robotic Assembly Tasks", // no
        // "An Approach of Cyber-Physical Production Systems Architecture for Robot Control", // no 
        // "An Architecture for Data Management, Visualisation and Supervision of Cyber- Physical Production Systems", // no
        // "An Automatic Software Behavior Model Generation Method for Industrial Cyber-Physical System", // no
        // "An Integrated Design Method For Cyber-Physical Production Systems", // no
        // "An Switchable Multi-resolution Architecture of Cyber-Physical Manufacturing Systems (CPMS) for Industrial Robots Collaboration", // no
        // "An architecture of an Intelligent Digital Twin in a Cyber-Physical Production System", // no
        // "Analyzing a ROS based architecture for its cross reuse in ISO26262 settings", // no
        // "Application framework of digital twin-driven product smart manufacturing system: A case study of aeroengine blade manufacturing", // no
        // "Application of Industrial Robot and Internet of Things in Intelligent Manufacturing System Supported by Software and Hardware", // no
        // "Application of the sequence planner control framework to an intelligent automation system with a focus on error handling", // ctrl
        // "Applying MDE to ROS systems: A comparative analysis", // ok
        // "Applying model-based Co-Simulation on modular Production Units in Complex Automation Systems", // no
        // "Approaches to Creating a Multi-agent Architecture in the Industrial Internet of Things Systems", // no
        // "Archer: An Event-Driven Architecture for Cyber-Physical Systems", // no
        // "Architecting Digital Twins", // no
        // "Architecting an agent-based fault diagnosis engine for IEC 61499 industrial cyber-physical systems", // no
        // "Architectural framework of digital twin-based cyber-physical production system for resilient rechargeable battery production", // no
        // "Architecture and knowledge modelling for self-organized reconfiguration management of cyber-physical production systems", // no
        // "Architecture for Digital Twin implementation focusing on Industry 4.0", // no 
        // "Assessing adaptability of software architectures for cyber physical production systems", // no
        // "Automatic Generation of Charging Point's Digital Twin for Virtual Commissioning of Their Automation Systems", // no
        // "Automation Software Architecture in CPPS - Definition, Challenges and Research Potentials", // no
        // "Behavior Trees based Flexible Task Planner Built on ROS2 Framework", // no
        // "Behavioral Analysis of ROS motion planners integrated with Robotics Middleware Framework (RMF)",  // no
        // "Co-simulation-based virtual commissioning for modular process plants: Requirements, framework and support-toolchain for a virtual automation testing environment", // no
        // "Codesign of Architecture, Control, and Scheduling of Modular Cyber-Physical Production Systems for Design Space Exploration", // no
        // "Combining self-organisation with decision-making and planning", // no
        // "CompROS: A composable ROS2 based architecture for real-time embedded robotic development", // no
        // "Concept Design of a System Architecture for a Manufacturing Cyber-physical Digital Twin System", // no
        // "Concept For An Interoperable Behavior Library For Virtual Commissioning Using PLCopenXML", // no
        // "Context-aware scheduling and control architecture for cyber-physical production systems", // no
        // "Control components for Collaborative and Intelligent Automation Systems", // ctrl
        // "Cooperative Control for Industrial Multi-agent Systems: Framework and Problems", // no
        // "Cooperative Robotics and Machine Learning for Smart Manufacturing: Platform Design and Trends Within the Context of Industrial Internet of Things", // no
        // "Cyber and Physical Systems Topology for the Industry 4.0 Smart Factory", // no
        // "Cyber-Physical Machine Tools towards Digitalisation and Servitisation of Manufacturing", // no
        // "Cyber-Physical Manufacturing Systems for Industry 4.0: Architectural Approach and Pilot Case", // no
        // "Cyber-Physical Production Systems supported by Intelligent Devices (SmartBoxes) for Industrial Processes Digitalization", // no
        // "Cyber-Physical System Implementation for Manufacturing With Analytics in the Cloud Layer", // no
        // "Cyber-Physical Systems for Industrial Applications", // no
        // "Cyber-physical microservices: An IoT-based framework for manufacturing systems",  // no
        // "Cyber-physical production systems architecture based on multi-agent's design pattern—comparison of selected approaches mapping four agent patterns", // no
        // "Cyber-physical production systems for SMEs-A generic multi agent based architecture and case study", // no
        // "Cyber-physical system architecture for machining production line", // no
        // "DINASORE: A dynamic intelligent reconfiguration tool for cyber-physical production systems", // no
        // "Data-driven framework to improve collaborative human-robot flexible manufacturing applications", // no
        // "Decentralized Autonomous Architecture for Resilient Cyber-Physical Production Systems", // no
        // "Decentralized Task and Path Planning for Multi-Robot Systems", // ctrl
        // "Demonstration of an industrial framework for an implementation of a process digital twin", // no
        // "Design Patterns in Building Software for a Middle Size League Robot Using Robot Operating System", // no
        // "Design and Development of a Research Oriented Low Cost Robotics Platform with a Novel Dynamic Global Path Planning Approach", // no
        // "Design and Implementation of Runtime Verification Framework for Cyber-Physical Production Systems", // no
        // "Design and Implementation of Software Based on Visual Simulation for Cargo Picking and Placing of Space Manipulator", // no
        // "Design of Flexible Cyber-Physical Production Systems Architecture for Industrial Robot Control", // no
        // "Design of a Framework for Implementation of Industrial Robot Manipulation Using PLC and ROS 2", // close...
        // "Design of a flexible robot cell demonstrator based on CPPS concepts and technologies", // ok
        // "Design of cyber physical system architecture for industry 4.0 through lean six sigma: conceptual foundations and research issues", // no
        // "Design patterns for the implementation of Industrial Agent-based AASs", // no
        // "Developing Production-Grade Applications with ROS 2", // no
        // "Developing a Testing Framework for Cyber-Physical Systems using Gazebo", // no
        // "Developing an Improved Software Architecture Framework for Smart Manufacturing", // no
        // "Developing an engineering tool for Cyber-Physical Production Systems", // close...
        // "Developing sensor signal-based digital twins for intelligent machine tools", // no
        // "Development and Deployment of Complex Robotic Applications using Containerized Infrastructures", // close...
        // "Development of Digital twin for Plug-and-Produce of a Machine tending system through ISO 21919 interface", // close...
        // "Development of Intelligent Behaviors for Social Robots via User-Friendly and Modular Programming Tools",
        // "Development of a structural framework to improve reconfigurable manufacturing system adoption in the manufacturing industry", // no
        // "Development of an Event-Driven System Architecture for Smart Manufacturing", // no
        // "Development of the Architecture and Reconfiguration Methods for the Smart, Self-Reconfigurable Manufacturing System", // no
        // "Digital Twin Enabled Smart Control Engineering as an Industrial AI: A New Framework and Case Study", // no
        // "Digital Twin Framework for Reconfigurable Manufacturing Systems: Challenges and Requirements", // no
        // "Digital Twin and Industrial Internet of Things Architecture to Reduce Carbon Emissions", // no
        // "Digital Twin as Industrial Robots Manipulation Validation Tool", // no
        // "Digital Twin: Toward the Integration Between System Design and RAMS Assessment Through the Model-Based Systems Engineering", // no
        // "Digital Twins Approach for Sustainable Industry", // no
        // "Digital twin based virtual commissioning for computerized numerical control machine tools", // ok
        // "Digital twin for adaptation of robots behavior in flexible robotic assembly lines", // no
        // "Digital twin for smart manufacturing: a review of concepts towards a practical industrial implementation", // no
        // "Digital twin framework for reconfigurable manufacturing systems (RMSs): design and simulation", // no
        // "Digital twin modeling method for CNC machine tool", // no
        // "Digital twin-based cyber physical production system architectural framework for personalized production", // no
        // "Digital twin-based industrial cloud robotics: Framework, control approach and implementation", // close ...
        // "Digital twin-driven virtual commissioning of machine tool", // close...
        // "Digital twins in manufacturing: An assessment of key features", // no
        // "Discrete Event Simulation as a Robust Supporting Tool for Smart Manufacturing", // no
        // "Distributed and Synchronized Setup towards Real-Time Robotic Control using ROS2 on Linux",  // no
        // "Documentation and Modeling of ROS Systems", // no
        // "Dynamic reconfiguration optimization of intelligent manufacturing system with human-robot collaboration based on digital twin", // no
        // "Dynamically Wiring CPPS Software Architectures", // no
        // "Efficient Industrial Solution for Robotic Task Sequencing Problem With Mutual Collision Avoidance & Cycle Time Optimization", // no
        // "Empowering The Eclipse Arrowhead Framework with a Digital Twin as a Proxy Service", // no
        // "Engineering Method and Tool for the Complete Virtual Commissioning of Robotic Cells", // ok
        // "Enhancing a Biological inspired Self-organized Architecture towards Smart Manufacturing", // no
        // "Evaluating Skill-Based Control Architecture for Flexible Automation Systems", // ctrl
        // "Evaluation of Cognitive Architectures for Cyber-Physical Production Systems", // no
        // "Event-Driven Programming of FPGA-accelerated ROS 2 Robotics Applications", // no
        // "Fault injection in Digital Twin as a means to test the response to process faults at virtual commissioning", // no
        // "Flexible Production Systems: Automated Generation of Operations Plans Based on ISA-95 and PDDL", // ok
        // "Flexible work cell simulator using digital twin methodology for highly complex systems in industry 4.0", // no
        // "Flow-Based ROS2 Programming Environment for Control Drone", // no
        // "Formal Verification of ROS Based Systems Using a Linear Logic Theorem Prover", // no
        // "Generating ROS-based Software for Industrial Cyber-Physical Systems from UML/MARTE", // ok
        // "Generating Safe Autonomous Decision-Making in ROS", // no 
        // "Generic Autonomic Management as a Service in a SOA-based Framework for Industry 4.0", // no
        // "Generic ROS-based Architecture for Heterogeneous Multi-Autonomous Systems Development", // no
        // "Generic digital twin architecture for industrial energy systems", // no
        // "How to build and validate a safe and reliable Autonomous Driving stack? A ROS based software modular architecture baseline", // no
        // "Hybrid Commissioning of Production Plants", // no
        // "Hybrid learning-based digital twin for manufacturing process: Modeling framework and implementation", // no
        // "INDUSTRIAL COLLABORATIVE ROBOT DIGITAL TWIN INTEGRATION AND CONTROL USING ROBOT OPERATING SYSTEM", // no
        // "Implementation of Digital Twin-based Virtual Commissioning in Machine Tool Manufacturing", // no
        // "Implementation of Industrial Cyber Physical System: Challenges and Solutions", // no
        // "Implementation of PLC controller connected Gazebo-ROS to support IEC 61131-3", // no
        // "Improving interoperability of Virtual Commissioning toolchains by using OPC-UA-based technologies", // no
        // "Industrial IoT and Digital Twins for a Smart Factory : An open source toolkit for application design and benchmarking", // no
        // "Integrated Cyber Physical Simulation Modelling Environment for Manufacturing 4.0", // no
        // "Integrated production and maintenance planning in cyber-physical production systems", // no
        // "Integrated virtual commissioning of a ROS2-based collaborative and intelligent automation system", // ok
        // "Integration challenges for the deployment of a multi-stage zero-defect manufacturing architecture", // no
        // "Integration of a formal specification approach into CPPS engineering workflow for machinery validation", // close...
        // "Intelligent Automation Framework Using AI and RPA: An Introduction", // no
        // "Intelligent Manufacturing with Digital Twin", // no
        // "Intelligent manufacturing system with human-cyber-physical fusion and collaboration for process fine control", // no
        "Interactive formal specification for efficient preparation of intelligent automation systems",
        "Internet of things based cyber-physical system framework for real-time operations",
        "Interoperability Between ROS and OPC UA: A Local Cloud-Based Approach",
        "Knowledge-based formal modeling for CPPS in personalized intelligent manufacturing",
        "KubeROS: A Unified Platform for Automated and Scalable Deployment of ROS2-based Multi-Robot Applications",
        "Lab-Scale Smart Factory Implementation Using ROS",
        "Let the Asset Decide: Digital Twins with Knowledge Graphs",
        "Let there be intelligence!- A novel cognitive architecture for teaching assistant social robots",
        "Leveraging domain specific modeling to increase accessibility of robot programming",
        "MAFOSS: Multi-Agent Framework using Open-Source Software",
        "Manufacturing machine virtual commissioning: Automated validation of the control software",
        "Mechatronic Swarm and its Virtual Commissioning",
        "Metamodeling of Cyber-Physical Production Systems using AutomationML for Collaborative Innovation",
        "Methodological Approach for Developing Reconfigurable Automation Systems",
        "Model-driven design and development of flexible automated production control configurations for industry 4.0",
        "Modeling and Optimization of Flexible Manufacturing Systems: A Stochastic Approach",
        "Modeling and object recognition skill transfer in industrial intelligent robots",
        "Modular Design Method for Reconfigurable Manufacturing Systems",
        "Modular ROS Based Autonomous Mobile Industrial Robot System for Automated Intelligent Manufacturing Applications",
        "Modular ROS-based software architecture for reconfigurable, Industry 4.0 compatible robotic workcells",
        "Modular System Design Approach for Cyber Physical Production Systems",
        "Modular Virtual Preparation method of production systems using a Digital Twin architecture",
        "Multi-Agent Systems to Implement Industry 4.0 Components",
        "Multi-Agent Technology for Industrial Applications: Barriers and Trends",
        "Multi-agent Based IEC 61499 Function Block Modelling for Distributed Intelligent Automation",
        "Multi-agent modeling of cyber-physical systems for IEC 61499 based distributed automation",
        "Multi-paradigm modelling and co-simulation in prototyping a cyber-physical production system",
        "New IT Driven Service-Oriented Smart Manufacturing: Framework and Characteristics",
        "Novel approach to establish model-based development and virtual commissioning in practice",
        "Optimized execution of PDDL plans using behavior trees",
        "P&P - Standard architecture to enable fast software prototyping for robot arms",
        "PRobPlan: A Framework of Integrating Probabilistic Planning into ROS",
        "Petri net controlled virtual commissioning - A virtual design-loop approach",
        "Placing the operator at the centre of Industry 4.0 design: Modelling and assessing human activities within cyber-physical systems",
        "PlanSys2: A Planning System Framework for ROS2",
        "Planning and acting framework under robot operating system",
        "Practical Guide to Smart Factory Transition Using IoT, Big Data and Edge Analytics",
        "Practical use of robot manipulators as intelligent manufacturing systems",
        "Pre-Integrated Architectures for sustainable complex Cyber-Physical Systems",
        "Probabilistic Planning for Robotics with ROSPlan",
        "Production Process Modelling Architecture to Support Improved Cyber-Physical Production Systems",
        "Programming Abstractions for Simulation and Testing on Smart Manufacturing Systems",
        "Property-based testing for the robot operating system",
        "Pushing ROS towards the Dark Side: A ROS-based Co-Simulation Architecture for Mixed-Reality Test Systems for Autonomous Vehicles",
        "RMAS architecture for industrial agents in IEC 61499",
        "ROS and cFS System (RACS): Easing Space Robotic Development",
        "ROS-Based Multi-Robot System Simulator",
        "ROS-Industrial based robotic cell for Industry 4.0: Eye-in-hand stereo camera and visual servoing for flexible, fast, and accurate picking and hooking in the production line",
        "ROS-MAGNA, a ROS-based framework for the definition and management of multi-UAS cooperative missions",
        "ROS-TiPlEx: How to make experts in A.I. Planning and Robotics talk together and be happy",
        "ROS-based architecture for fast digital twin development of smart manufacturing robotized systems",
        "ROS-based robot simulation in human-robot collaboration",
        "ROSIE: A ROS Adapter for a Modular Digital Twinning Framework",
        "Real-time simulation and virtual commissioning of a modular robot system with OPC UA",
        "Relaxing Platform Dependencies in Agent-Based Control Systems",
        "Repeatable Decentralized Simulations for Cyber-Physical Systems",
        "Requirements for designing and controlling autonomous collaborative robots system-an industrial case",
        "Research on Intelligent Manufacturing System Model Based on Programmable Logic Controller",
        "Research on real-time status visualization of intelligent manufacturing production line based on digital twin",
        "Research on the Architecture of Cyber-Physical Machine Tool System",
        "Resilience enhancement through a multi-agent approach over cyber-physical systems",
        "Resilient architecture for cyber-physical production systems",
        "RoBMEX: ROS-based modelling framework for end-users and experts",
        "RoSA: A Framework for Modeling Self-Awareness in Cyber-Physical Systems",
        "RobotAtFactory 4.0: a ROS framework for the SimTwo simulator",
        "SDCWorks: A Formal Framework for Software Defined Control of Smart Manufacturing Systems",
        "SWAP-IT: A Scalable and Lightweight Industry 4.0 Architecture for Cyber-Physical Production Systems",
        "Safe and secure system architectures for cyber-physical systems",
        "Semi-automatic generation of a virtual representation of a production cell: Combining 3D CAD and VDI-2860 behavior models by means of AutomationML",
        "Sequence Planner: A Framework for Control of Intelligent Automation Systems",
        "Simulation Environment for Modeling and Testing of Autonomous Assembly in Space for Multiple Robotic Arms",
        "Simulation and virtual commissioning for intelligent buffer control system - Case study",
        "Simulation framework for cyber-physical production system: Applying concept of LVC interoperation",
        "Skill-Based Easy Programming Interface for Industrial Applications",
        "Software-in-the-Loop Modeling and Simulation Framework for Autonomous Vehicles",
        "Standardized Framework for Evaluating Centralized and Decentralized Control Systems in Modular Assembly Systems",
        "Streamline 3D simulation model development for virtual commissioning with IEC61499",
        "Supervisory Control of Reconfigurable Manufacturing Systems based on NCESs",
        "Supporting Skill-based Flexible Manufacturing with Symbolic AI Methods",
        "Supporting the Design, Commissioning and Supervision of Smart Factory Components through their Digital Twin",
        "Synchronizing digital process twins between virtual products and resources - A virtual design method",
        "System Architectures for Cyber-Physical Production Systems enabling Self-X and Autonomy",
        "Teaching robotics with robot operating system (ROS): A behavior model perspective",
        "Testing Cyber-Physical Production System: Test Methods Categorization and dataset",
        "The Development of a Digital Twin Framework for an Industrial Robotic Drilling Process",
        "The High-Assurance ROS Framework",
        "The PoundCloud framework for ROS-based cloud robotics: Case studies on autonomous navigation and human-robot interaction",
        "The architecture development of Industry 4.0 compliant smart machine tool system (SMTS)",
        "The component-based design method for agent-based multi-AGV system",
        "The framework design of smart factory in discrete manufacturing industry based on cyber-physical system",
        "TiLA: Twin-in-the-Loop Architecture for Cyber-Physical Production Systems",
        "Toward Intelligent Cyber-Physical Systems: Digital Twin Meets Artificial Intelligence",
        "Toward Smart Manufacturing Using Spiral Digital Twin Framework and Twinchain",
        "Towards IEC 61499 Based Distributed Intelligent Automation: Design and Computing Perspectives",
        "Towards IEC 61499-Based Distributed Intelligent Automation: A Literature Review",
        "Towards Integrating Multi-Agent Organizations in OPC UA for Developing Adaptive Cyber-Physical Systems",
        "Towards a Cloud-Based Controller for Data-Driven Service Orchestration in Smart Manufacturing",
        "Towards a Novel Software Framework for the Intuitive Generation of Process Flows for Multiple Robotic Systems",
        "Towards a digital twin platform for industrie 4.0",
        "Towards a real-time BDI model for ROS 2",
        "Towards an OPC UA Compliant Programming Approach with Formal Model of Computation for Dynamic Reconfigurable Automation Systems",
        "Towards cloud-based virtual commissioning of distributed automation applications with IEC 61499 and containerization technology",
        "Towards the adoption of smart manufacturing systems: A development framework",
        "Using a model-based engineering approach for developing Industrial Internet of Things applications",
        "Utilizing an Enterprise Architecture Framework for Model-Based Industrial Systems Engineering",
        "Validation of dynamic interoperability and virtual commissioning of production equipment in early development stages",
        "Value Creation with Digital Twins: Application-Oriented Conceptual Framework and Case Study",
        "Verification of system-wide safety properties of ROS applications",
        "Virtual Commissioning Approach based on the Discrete Element Method",
        "Virtual Commissioning Simulation as OpenAI Gym - A Reinforcement Learning Environment for Control Systems",
        "Virtual Engineering and Commissioning to Support the Lifecycle of a Manufacturing Assembly System",
        "Virtual Testing in Automated Driving Systems Certification. A Longitudinal Dynamics Validation Example",
        "Virtual comissioning of manufacturing system intelligent control",
        "Virtual commissioning as the main core of industry 4.0 - Case study in the automotive paint shop",
        "Virtual commissioning for advanced manufacturing using digital tools",
        "Virtual commissioning of a robotic cell: an educational case study",
        "Virtual commissioning of a robotized production cell with use of mechatronic features",
        "Virtual experimental investigation for industrial robotics in gazebo environment",
        "simbIoTe: A Simulator for Building Cyber-Physical System and Internet of Things Environments",
    );

    let stage_3 = vec!(
        "A Behaviour-Driven Development Approach for Cyber-Physical Production Systems",
        "A Formal Model-Based Design Method for Robotic Systems",
        "A Container-Based Framework for Developing ROS Applications",
        "A Framework for Multidisciplinary Simulation of Cyber-Physical Production Systems", // close...
        "A Framework for Service-Oriented Digital Twin Systems for Discrete Workshops and Its Practical Case Study",
        "A Novel Implementation Framework of Digital Twins for Intelligent Manufacturing Based on Container Technology and Cloud Manufacturing Services",
        "A Unified Digital Twin Framework for Real-time Monitoring and Evaluation of Smart Manufacturing Systems", // close...
        "A framework for an effective virtual commissioning of agent-based cyber-physical production systems integrated into manufacturing facilities",
        "A hybrid modeling methodology for cyber physical production systems: framework and key techniques",
        "A product-process-resource based formal modelling framework for customized manufacturing in cyber-physical production systems",
        "Advanced tools for the control engineer in Industry 4.0",
        "Applying MDE to ROS systems: A comparative analysis",
        "Design of a Framework for Implementation of Industrial Robot Manipulation Using PLC and ROS 2", // close...
        "Design of a flexible robot cell demonstrator based on CPPS concepts and technologies",
        "Developing an engineering tool for Cyber-Physical Production Systems", // close...
        "Development and Deployment of Complex Robotic Applications using Containerized Infrastructures", // close...
        "Development of Digital twin for Plug-and-Produce of a Machine tending system through ISO 21919 interface", // close...
        "Development of Intelligent Behaviors for Social Robots via User-Friendly and Modular Programming Tools",
        "Digital twin based virtual commissioning for computerized numerical control machine tools",
        "Digital twin-based industrial cloud robotics: Framework, control approach and implementation", // close...
        "Digital twin-driven virtual commissioning of machine tool", // close...
        "Engineering Method and Tool for the Complete Virtual Commissioning of Robotic Cells",
        "Flexible Production Systems: Automated Generation of Operations Plans Based on ISA-95 and PDDL",
        "Generating ROS-based Software for Industrial Cyber-Physical Systems from UML/MARTE",
        "Integrated virtual commissioning of a ROS2-based collaborative and intelligent automation system",
        "Integration of a formal specification approach into CPPS engineering workflow for machinery validation", // close...


    );

    let stage_3_control = vec!(
        "A Demonstration of BDI-Based Robotic Systems with ROS2",
        "A Formal Model-Based Design Method for Robotic Systems",
        "A Generic Plug & Produce System Composed of Semantic OPC UA Skills",
        "A Middleware Platform for Intelligent Automation: An Industrial Prototype Implementation",
        "A Software Architecture for Service Robots Manipulating Objects in Human Environments",
        "Application of the sequence planner control framework to an intelligent automation system with a focus on error handling",
        "Control components for Collaborative and Intelligent Automation Systems",
        "Decentralized Task and Path Planning for Multi-Robot Systems",
        "Evaluating Skill-Based Control Architecture for Flexible Automation Systems",

    );

    let stage_3_vc = vec!(
        "A Virtual Commissioning Selection Approach for Machine Automation",
        "Empowering The Eclipse Arrowhead Framework with a Digital Twin as a Proxy Service"
    );

    let for_the_adversary_journal = vec!(
        "A Digital Twin Based Industrial Automation and Control System Security Architecture",
        "Fault injection in Digital Twin as a means to test the response to process faults at virtual commissioning",
    );

}
