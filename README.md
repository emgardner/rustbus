A modbus TCP and RTU master that allows for:
- Creation of Register Tables
- Saving Configuration
- Easy Request Building


The project is built using rust and leverages:

[The Iced GUI Framework](https://github.com/iced-rs/iced)

[tokio-modbus](https://github.com/slowtec/tokio-modbus)


# TODO

## Application Layer
- [ ] Add ability to view raw data across the wire will need to modify tokio-modbus for this.
- [ ] Add ability to perform multiple Register Read's and Write's within a single action and then save the action.
- [ ] Add Formatting/Decoding to all number's Float/Hex/LED/BIN
- [ ] Clean up messaging architecture
- [ ] Clean up error handling
- [ ] Add protocol addressing
- [ ] Add plotting (nice to have)?

## Tables
- [ ] Move row up or down
- [ ] Sort rows
- [ ] When row clicked automatically change the request
- [ ] Restrict Actions based on row type

## Request 
- [ ] Add Support for all and custom
- [ ] Fix height and width of container

## Styling
- [ ] Add more consistently styled icons (line weights are all different currently)
- [ ] Custom Font
- [ ] Change Slave ID 







