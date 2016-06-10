const routing = {
    "/": {
        template: "splash",
    },
};

_.each(routing, (options, path) => {
    Router.route(path, function() {
        this.render(options.template, _.omit(options, "template"));
    });
});
