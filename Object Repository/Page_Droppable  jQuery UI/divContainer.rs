<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>divContainer</name>
   <tag></tag>
   <elementGuidId>5901b9b5-7cff-498c-a7d9-c7ae6db7f9de</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='container']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#container</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>e91c790b-f832-4549-9b10-452007736452</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>container</value>
      <webElementGuid>1dcb7e8c-dbdb-4211-9b08-0857fa6ab6d3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	
		jQuery UI

		
			
		
	

	
		
	
Demos
Download
API Documentation
Themes
Development
Support
Blog
About
	Navigate...DemosDownloadAPI DocumentationThemesDevelopmentSupportBlogAbout


		
	search
	
		Search jQuery UI
		
	

	

	



	
				Droppable
		
		
		
Create targets for draggable elements.

ExamplesDefault functionalityAcceptPrevent propagationRevert draggable positionSimple photo managerVisual feedback
Enable any DOM element to be droppable, a target for draggable elements.
 view source
	
		
			
				
				
					
						1
					
						2
					
						3
					
						4
					
						5
					
						6
					
						7
					
						8
					
						9
					
						10
					
						11
					
						12
					
						13
					
						14
					
						15
					
						16
					
						17
					
						18
					
						19
					
						20
					
						21
					
						22
					
						23
					
						24
					
						25
					
						26
					
						27
					
						28
					
						29
					
						30
					
						31
					
						32
					
						33
					
						34
					
						35
					
						36
					
						37
					
						38
					
						39
					
						40
					
						41
					
				
				
				
					&lt;!doctype html>&lt;html lang=&quot;en&quot;>&lt;head>  &lt;meta charset=&quot;utf-8&quot;>  &lt;meta name=&quot;viewport&quot; content=&quot;width=device-width, initial-scale=1&quot;>  &lt;title>jQuery UI Droppable - Default functionality&lt;/title>  &lt;link rel=&quot;stylesheet&quot; href=&quot;//code.jquery.com/ui/1.13.2/themes/base/jquery-ui.css&quot;>  &lt;link rel=&quot;stylesheet&quot; href=&quot;/resources/demos/style.css&quot;>  &lt;style>  #draggable { width: 100px; height: 100px; padding: 0.5em; float: left; margin: 10px 10px 10px 0; }  #droppable { width: 150px; height: 150px; padding: 0.5em; float: left; margin: 10px; }  &lt;/style>  &lt;script src=&quot;https://code.jquery.com/jquery-3.6.0.js&quot;>&lt;/script>  &lt;script src=&quot;https://code.jquery.com/ui/1.13.2/jquery-ui.js&quot;>&lt;/script>  &lt;script>  $( function() {    $( &quot;#draggable&quot; ).draggable();    $( &quot;#droppable&quot; ).droppable({      drop: function( event, ui ) {        $( this )          .addClass( &quot;ui-state-highlight&quot; )          .find( &quot;p&quot; )            .html( &quot;Dropped!&quot; );      }    });  } );  &lt;/script>&lt;/head>&lt;body> &lt;div id=&quot;draggable&quot; class=&quot;ui-widget-content&quot;>  &lt;p>Drag me to my target&lt;/p>&lt;/div> &lt;div id=&quot;droppable&quot; class=&quot;ui-widget-header&quot;>  &lt;p>Drop here&lt;/p>&lt;/div>  &lt;/body>&lt;/html>
				
			
		
	



Want to learn more about the droppable interaction? Check out the
API documentation.	
	
	
		Interactions
		
			DraggableDroppableResizableSelectableSortable		
	
	
		Widgets
		
			AccordionAutocompleteButtonCheckboxradioControlgroupDatepickerDialogMenuProgressbarSelectmenuSliderSpinnerTabsTooltip		
	
	
		Effects
		
			Add ClassColor AnimationEasingEffectHideRemove ClassShowSwitch ClassToggleToggle Class		
	
	
		Utilities
		
			PositionWidget Factory		
	



	
</value>
      <webElementGuid>ba1daf9d-b42f-4396-b17a-5841181923a4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;container&quot;)</value>
      <webElementGuid>69fa552c-5616-41db-82ba-7c41ada9023f</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='container']</value>
      <webElementGuid>7d15575c-82e4-41b0-aaac-b1d2c49c3f7a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Donate'])[1]/following::div[1]</value>
      <webElementGuid>243d0b29-9ea9-4a15-934c-c7a3effaf88d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Conduct'])[1]/following::div[1]</value>
      <webElementGuid>03fe1228-2cd6-4597-8711-45ac6d5737d1</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body/div</value>
      <webElementGuid>287f3a2a-8ba2-4ff9-9a16-8923a8ec60a1</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'container' and (text() = '
	
		jQuery UI

		
			
		
	

	
		
	
Demos
Download
API Documentation
Themes
Development
Support
Blog
About
	Navigate...DemosDownloadAPI DocumentationThemesDevelopmentSupportBlogAbout


		
	search
	
		Search jQuery UI
		
	

	

	



	
				Droppable
		
		
		
Create targets for draggable elements.

ExamplesDefault functionalityAcceptPrevent propagationRevert draggable positionSimple photo managerVisual feedback
Enable any DOM element to be droppable, a target for draggable elements.
 view source
	
		
			
				
				
					
						1
					
						2
					
						3
					
						4
					
						5
					
						6
					
						7
					
						8
					
						9
					
						10
					
						11
					
						12
					
						13
					
						14
					
						15
					
						16
					
						17
					
						18
					
						19
					
						20
					
						21
					
						22
					
						23
					
						24
					
						25
					
						26
					
						27
					
						28
					
						29
					
						30
					
						31
					
						32
					
						33
					
						34
					
						35
					
						36
					
						37
					
						38
					
						39
					
						40
					
						41
					
				
				
				
					&lt;!doctype html>&lt;html lang=&quot;en&quot;>&lt;head>  &lt;meta charset=&quot;utf-8&quot;>  &lt;meta name=&quot;viewport&quot; content=&quot;width=device-width, initial-scale=1&quot;>  &lt;title>jQuery UI Droppable - Default functionality&lt;/title>  &lt;link rel=&quot;stylesheet&quot; href=&quot;//code.jquery.com/ui/1.13.2/themes/base/jquery-ui.css&quot;>  &lt;link rel=&quot;stylesheet&quot; href=&quot;/resources/demos/style.css&quot;>  &lt;style>  #draggable { width: 100px; height: 100px; padding: 0.5em; float: left; margin: 10px 10px 10px 0; }  #droppable { width: 150px; height: 150px; padding: 0.5em; float: left; margin: 10px; }  &lt;/style>  &lt;script src=&quot;https://code.jquery.com/jquery-3.6.0.js&quot;>&lt;/script>  &lt;script src=&quot;https://code.jquery.com/ui/1.13.2/jquery-ui.js&quot;>&lt;/script>  &lt;script>  $( function() {    $( &quot;#draggable&quot; ).draggable();    $( &quot;#droppable&quot; ).droppable({      drop: function( event, ui ) {        $( this )          .addClass( &quot;ui-state-highlight&quot; )          .find( &quot;p&quot; )            .html( &quot;Dropped!&quot; );      }    });  } );  &lt;/script>&lt;/head>&lt;body> &lt;div id=&quot;draggable&quot; class=&quot;ui-widget-content&quot;>  &lt;p>Drag me to my target&lt;/p>&lt;/div> &lt;div id=&quot;droppable&quot; class=&quot;ui-widget-header&quot;>  &lt;p>Drop here&lt;/p>&lt;/div>  &lt;/body>&lt;/html>
				
			
		
	



Want to learn more about the droppable interaction? Check out the
API documentation.	
	
	
		Interactions
		
			DraggableDroppableResizableSelectableSortable		
	
	
		Widgets
		
			AccordionAutocompleteButtonCheckboxradioControlgroupDatepickerDialogMenuProgressbarSelectmenuSliderSpinnerTabsTooltip		
	
	
		Effects
		
			Add ClassColor AnimationEasingEffectHideRemove ClassShowSwitch ClassToggleToggle Class		
	
	
		Utilities
		
			PositionWidget Factory		
	



	
' or . = '
	
		jQuery UI

		
			
		
	

	
		
	
Demos
Download
API Documentation
Themes
Development
Support
Blog
About
	Navigate...DemosDownloadAPI DocumentationThemesDevelopmentSupportBlogAbout


		
	search
	
		Search jQuery UI
		
	

	

	



	
				Droppable
		
		
		
Create targets for draggable elements.

ExamplesDefault functionalityAcceptPrevent propagationRevert draggable positionSimple photo managerVisual feedback
Enable any DOM element to be droppable, a target for draggable elements.
 view source
	
		
			
				
				
					
						1
					
						2
					
						3
					
						4
					
						5
					
						6
					
						7
					
						8
					
						9
					
						10
					
						11
					
						12
					
						13
					
						14
					
						15
					
						16
					
						17
					
						18
					
						19
					
						20
					
						21
					
						22
					
						23
					
						24
					
						25
					
						26
					
						27
					
						28
					
						29
					
						30
					
						31
					
						32
					
						33
					
						34
					
						35
					
						36
					
						37
					
						38
					
						39
					
						40
					
						41
					
				
				
				
					&lt;!doctype html>&lt;html lang=&quot;en&quot;>&lt;head>  &lt;meta charset=&quot;utf-8&quot;>  &lt;meta name=&quot;viewport&quot; content=&quot;width=device-width, initial-scale=1&quot;>  &lt;title>jQuery UI Droppable - Default functionality&lt;/title>  &lt;link rel=&quot;stylesheet&quot; href=&quot;//code.jquery.com/ui/1.13.2/themes/base/jquery-ui.css&quot;>  &lt;link rel=&quot;stylesheet&quot; href=&quot;/resources/demos/style.css&quot;>  &lt;style>  #draggable { width: 100px; height: 100px; padding: 0.5em; float: left; margin: 10px 10px 10px 0; }  #droppable { width: 150px; height: 150px; padding: 0.5em; float: left; margin: 10px; }  &lt;/style>  &lt;script src=&quot;https://code.jquery.com/jquery-3.6.0.js&quot;>&lt;/script>  &lt;script src=&quot;https://code.jquery.com/ui/1.13.2/jquery-ui.js&quot;>&lt;/script>  &lt;script>  $( function() {    $( &quot;#draggable&quot; ).draggable();    $( &quot;#droppable&quot; ).droppable({      drop: function( event, ui ) {        $( this )          .addClass( &quot;ui-state-highlight&quot; )          .find( &quot;p&quot; )            .html( &quot;Dropped!&quot; );      }    });  } );  &lt;/script>&lt;/head>&lt;body> &lt;div id=&quot;draggable&quot; class=&quot;ui-widget-content&quot;>  &lt;p>Drag me to my target&lt;/p>&lt;/div> &lt;div id=&quot;droppable&quot; class=&quot;ui-widget-header&quot;>  &lt;p>Drop here&lt;/p>&lt;/div>  &lt;/body>&lt;/html>
				
			
		
	



Want to learn more about the droppable interaction? Check out the
API documentation.	
	
	
		Interactions
		
			DraggableDroppableResizableSelectableSortable		
	
	
		Widgets
		
			AccordionAutocompleteButtonCheckboxradioControlgroupDatepickerDialogMenuProgressbarSelectmenuSliderSpinnerTabsTooltip		
	
	
		Effects
		
			Add ClassColor AnimationEasingEffectHideRemove ClassShowSwitch ClassToggleToggle Class		
	
	
		Utilities
		
			PositionWidget Factory		
	



	
')]</value>
      <webElementGuid>4ac14fb9-1339-4d04-9061-a52c1f2b355b</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
