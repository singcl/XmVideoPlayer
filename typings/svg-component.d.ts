
declare module '~virtual/svg-component' {
  const XmSvgIcon: import("vue").DefineComponent<{
      name: {
          type: import("vue").PropType<"icon-feedback" | "icon-common-feedback">;
          default: string;
          required: true;
      };
  }, {}, unknown, {}, {}, import("vue").ComponentOptionsMixin, import("vue").ComponentOptionsMixin, {}, string, import("vue").VNodeProps & import("vue").AllowedComponentProps & import("vue").ComponentCustomProps, Readonly<import("vue").ExtractPropTypes<{
      name: {
          type: import("vue").PropType<"icon-feedback" | "icon-common-feedback">;
          default: string;
          required: true;
      };
  }>>, {
      name: "icon-feedback" | "icon-common-feedback";
  }>;
  export const svgNames: ["icon-feedback" , "icon-common-feedback"];
  export type SvgName = "icon-feedback" | "icon-common-feedback";
  export default XmSvgIcon;
}
